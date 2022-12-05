#![allow(dead_code, unused_variables)]

use std::{collections::VecDeque, str::FromStr, string::ParseError};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day5.txt")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

#[derive(Debug)]
pub struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn parse_instructions(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .skip_while(|l| !l.contains("1"))
            .skip(2)
            .map(|s| s.parse().unwrap())
            .collect_vec()
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(" ").collect::<Vec<_>>();
        let mut instruction = Instruction {
            from: s[3].parse().unwrap(),
            to: s[5].parse().unwrap(),
            quantity: s[1].parse().unwrap(),
        };
        instruction.from = instruction.from - 1;
        instruction.to = instruction.to - 1;
        Ok(instruction)
    }
}

pub trait Crane {
    fn handle_instruction(&mut self, instruction: &Instruction);
    fn get_top_crates(&self) -> String;
}

struct Crane9001 {
    cargo: Vec<VecDeque<char>>,
}

impl Crane for Crane9001 {
    fn handle_instruction(&mut self, instruction: &Instruction) {

        let mut tmp = vec![];

        for q in 0..instruction.quantity {
            tmp.push(self.cargo[instruction.from].pop_front().unwrap())
        }

        for item in tmp.iter().rev() {
            self.cargo[instruction.to].push_front(*item)
        }
    }

    fn get_top_crates(&self) -> String {
        self.cargo.iter().filter_map(|m| m.front()).join("")
    }
}

#[derive(Debug)]
pub struct SimpleCrane {
    cargo: Vec<VecDeque<char>>,
}

impl Crane for SimpleCrane {
    fn handle_instruction(&mut self, instruction: &Instruction) {
        for i in 0..instruction.quantity {
            let item = self.cargo[instruction.from].pop_front().unwrap();
            self.cargo[instruction.to].push_front(item);
        }
    }

    fn get_top_crates(&self) -> String {
        self.cargo.iter().filter_map(|m| m.front()).join("")
    }
}

fn crane_parser(input: &str) -> Vec<VecDeque<char>> {
    let cols = (input.lines().next().unwrap().len() + 1) / 4;

    let mut cargo: Vec<VecDeque<char>> = input
        .lines()
        .take_while(|l| !l.contains("1"))
        .fold(vec![VecDeque::new(); cols], |mut acc, line| {
            for i in 0..(cols) {
                acc[i].push_back(line.as_bytes()[(i * 4 + 1)] as char)
            }
            acc
        })
        .clone();

    cargo.iter_mut().for_each(|c| c.retain(|p| *p != ' '));
    cargo
}

impl FromStr for SimpleCrane {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cargo = crane_parser(s);

        Ok(SimpleCrane { cargo })
    }
}

impl FromStr for Crane9001 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cargo = crane_parser(s);
        Ok(Crane9001 { cargo })
    }
}

fn part1(input: &str) -> anyhow::Result<String> {
    let mut simple_crane = input.parse::<SimpleCrane>().unwrap();
    let instructions = Instruction::parse_instructions(input);

    instructions
        .iter()
        .for_each(|instruction| simple_crane.handle_instruction(instruction));

    Ok(simple_crane.get_top_crates())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let mut crane_9001 = input.parse::<Crane9001>().unwrap();
    let instructions = Instruction::parse_instructions(input);

    instructions
        .iter()
        .for_each(|instruction| crane_9001.handle_instruction(instruction));

    Ok(crane_9001.get_top_crates())
}

#[cfg(test)]
mod test {
    use crate::{part1, SimpleCrane};

    #[test]
    pub fn p1() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let crane: SimpleCrane = input.parse().unwrap();
        println!("{:?}", crane);

        assert_eq!(part1(input).unwrap(), "CMZ");
    }
}
