fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let input = std::fs::read_to_string("input/day2.txt")?;

    let input: Vec<(&str, &str)> = input.lines().map(|l| l.split_once(" ").unwrap()).collect();

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &Vec<(&str, &str)>) -> Result<(), Box<dyn std::error::Error>> { 

    let mut total_score = 0;
    
    // A: ROCK
    // B: PAPER
    // C: SCISSOR,

    // X: Rock
    // Y: Paper
    // Z: Scissor,

    for players in input { 
        let game_score = match players { 
            ("A", "X") => 3 + 1,
            ("B", "X") => 0 + 1,
            ("C", "X") => 6 + 1,
            ("A", "Y") => 6 + 2,
            ("B", "Y") => 3 + 2,
            ("C", "Y") => 0 + 2,
            ("A", "Z") => 0 + 3,
            ("B", "Z") => 6 + 3,
            ("C", "Z") => 3 + 3,
            _ => unreachable!("Should not happen! BAD ELF!")
        };

        total_score = game_score + total_score;
    }

    println!("{}", total_score); 
    Ok(())
}

fn part2(input: &Vec<(&str, &str)>) -> Result<(), Box<dyn std::error::Error>> { 

    let input = std::fs::read_to_string("input/day2.txt")?;

    let input: Vec<(&str, &str)> = input.lines().map(|l| l.split_once(" ").unwrap()).collect();

    let mut total_score = 0;
    
    // A: ROCK
    // B: PAPER
    // C: SCISSOR,

    // X: Rock
    // Y: Paper
    // Z: Scissor,

    for players in input { 
        let game_score = match players { 
            ("A", "X") => 0 + 3,
            ("B", "X") => 0 + 1,
            ("C", "X") => 0 + 2,
            ("A", "Y") => 3 + 1,
            ("B", "Y") => 3 + 2,
            ("C", "Y") => 3 + 3,
            ("A", "Z") => 6 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "Z") => 6 + 1,
            _ => unreachable!("Should not happen! BAD ELF!")
        };

        total_score = game_score + total_score;
    }

    println!("{}", total_score); 

    Ok(())
}