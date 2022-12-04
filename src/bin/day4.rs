#![allow(dead_code, unused_variables)]
use std::{str::FromStr, string::ParseError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day4.txt")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

struct Section {
    pub start: i32,
    pub end: i32,
    pub size: i32,
}

impl Section {
    pub fn contains(&self, other: &Section) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    // Smallest start always self
    pub fn overlaps(&self, other: &Section) -> bool { 
        other.start <= self.end
    }
}

impl FromStr for Section {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once("-").unwrap();
        let start = s.0.parse().unwrap();
        let end = s.1.parse().unwrap();

        Ok(Self {
            start,
            end,
            size: end - start,
        })
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = input
        .lines()
        .map(|s| {
            let sections = s.split_once(",").unwrap();
            let s1 = sections.0.trim().parse::<Section>().unwrap();
            let s2 = sections.1.trim().parse::<Section>().unwrap();
            let mut o = vec![s1, s2];
            o.sort_by_key(|s| -s.size);
            o[0].contains(&o[1])
        })
        .filter(|r| *r)
        .count();

    Ok(input)
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = input
    .lines()
    .map(|s| {
        let sections = s.split_once(",").unwrap();
        let s1 = sections.0.trim().parse::<Section>().unwrap();
        let s2 = sections.1.trim().parse::<Section>().unwrap();
        let mut o = vec![s1, s2];
        o.sort_by_key(|s| s.start);
        o[0].overlaps(&o[1])
    })
    .filter(|r| *r)
    .count();

    Ok(input)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn day1() {
        let input = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;

        assert_eq!(part1(input).unwrap(), 2);
    }

    #[test]
    fn day2() {
        let input = r#"5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        4-6,7-10"#;

        assert_eq!(part2(input).unwrap(),  4)
    }
}
