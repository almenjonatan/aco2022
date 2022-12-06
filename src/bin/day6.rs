#![allow(dead_code, unused_variables)]

use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day6.txt")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let output = 0;

    let c = input.chars().collect::<Vec<_>>();

    for i in 4..c.len() {
        let window = &c[(i - 4)..(i)];
        let mut h: HashSet<char> = HashSet::new();
        h.extend(window);
        if (h.len()) == 4 {
            return Ok(i);
        }
    }

    println!();

    Ok(output)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let output = 0;

    let c = input.chars().collect::<Vec<_>>();

    for i in 14..c.len() {
        let window = &c[(i - 14)..(i)];
        let mut h: HashSet<char> = HashSet::new();
        h.extend(window);
        if (h.len()) == 14 {
            return Ok(i);
        }
    }

    Ok(output)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    #[ignore = "reason"]
    pub fn p1() {
        let input: Vec<(_, usize)> = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for i in input {
            assert_eq!(part1(i.0).unwrap(), i.1);
        }
    }

    #[test]
    pub fn p2() {
        let input: Vec<(_, usize)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for i in input {
            assert_eq!(part2(i.0).unwrap(), i.1);
        }
    }
}
