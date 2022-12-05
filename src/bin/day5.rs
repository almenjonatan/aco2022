#![allow(dead_code, unused_variables)]

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day5.txt")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<Instruction>, Vec<Vec<&str>>) {
    let crate_positions = input
        .lines()
        .find_position(|l| l.contains("1"))
        .unwrap()
        .1
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .last()
        .unwrap() as usize;

    let mut cargo: Vec<Vec<&str>> = input
        .lines()
        .by_ref()
        .take_while(|l| !l.contains("1"))
        .fold(vec![vec![]; crate_positions], |mut acc, line| {
            for i in 0..crate_positions {
                let k = (i * 4) as usize;
                acc[i].push(&line[(k + 1)..(k + 2)])
            }
            acc
        });

    cargo.iter_mut().for_each(|c| c.retain(|p| *p != " "));

    let instructions = input
        .lines()
        .skip_while(|l| !l.contains("1"))
        .skip(2)
        .map(|s| {
            let s = s.split(" ").collect::<Vec<_>>();
            let mut i = Instruction {
                from: s[3].parse().unwrap(),
                to: s[5].parse().unwrap(),
                quantity: s[1].parse().unwrap(),
            };
            i.from = i.from - 1;
            i.to = i.to - 1;
            i
        })
        .collect_vec();

    (instructions, cargo)
}

fn part1(input: &str) -> anyhow::Result<String> {
    let (instructions, mut cargo) = parse(input);

    for i in instructions {
        for q in 0..i.quantity {
            let item = cargo[i.from].remove(0);
            cargo[i.to].insert(0, item);
        }
    }

    let s = cargo.iter().filter_map(|m| m.first()).join("");
    Ok(s)
}

fn part2(input: &str) -> anyhow::Result<String> {
    let (instructions, mut cargo) = parse(input);

    for i in instructions {
        if i.quantity == 1 {
            let item = cargo[i.from].remove(0);
            cargo[i.to].insert(0, item)
        }

        if i.quantity > 1 {
            let mut tmp = vec![];

            for q in 0..i.quantity {
                tmp.push(cargo[i.from].remove(0))
            }

            for item in tmp.iter().rev() {
                cargo[i.to].insert(0, item)
            }
        }
    }

    let s = cargo.iter().filter_map(|m| m.first()).join("");

    Ok(s)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    pub fn p1() {
        let input= r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        assert_eq!(part1(input).unwrap(), "CMZ");
    }
}
