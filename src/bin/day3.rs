use std::{collections::HashSet, str::Chars};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day3.txt")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let compartments: Vec<(&str, &str)> = input.lines().map(|l| l.split_at(l.len() / 2)).collect();

    let mut score = 0;

    for (compartment1, compartment2) in compartments {
        let mut c1 = HashSet::new();
        let mut c2 = HashSet::new();

        c1.extend(compartment1.chars());
        c2.extend(compartment2.chars());

        for c in c2 {
            //already exists
            if !c1.insert(c) {
                score = char_to_priority(&c) + score;
                break;
            }
        }
    }

    println!("{}", score);

    Ok(score)
}

fn part2(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut compartments = input.lines();

    let mut sum = 0;

    loop {
        let taken = compartments.by_ref().take(3);
        let s: Vec<Chars> = taken.map(|c| c.chars()).collect();

        if s.len() == 0 {
            break;
        }
        sum = sum + char_to_priority(&similar(s))
    }

    Ok(sum)
}

fn similar(containers: Vec<Chars>) -> char {
    let containers: Vec<HashSet<char>> = containers
        .iter()
        .map(|c| {
            let mut h = HashSet::new();
            h.extend(c.clone());
            h
        })
        .collect();

    for c in containers.get(0).unwrap() {
        let token = containers.iter().skip(1).all(|other| other.contains(&c));

        if token {
            return *c;
        }
    }

    unreachable!("Should not happen")
}

fn char_to_priority(c: &char) -> i32 {
    match c.is_lowercase() {
        true => (*c as i32) - 96,
        false => (*c as i32) - 38,
    }
}

#[cfg(test)]
pub mod test {
    use crate::{part1, part2};

    #[test]
    pub fn p1() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let score = part1(&s).unwrap();

        assert_eq!(score, 16);
    }

    #[test]
    pub fn p2() {
        let s = r"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg";

        let score = part2(&s).unwrap();
        assert_eq!(score, 18);

        let s = r"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let score = part2(&s).unwrap();
        assert_eq!(score, 52);
    }
}
