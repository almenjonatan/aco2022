fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day1.txt")?;

    let mut elf_calories: Vec<i32> = vec![];
    let mut current_elf: i32 = 0;

    for l in input.lines() {
        if l.is_empty() {
            elf_calories.push(current_elf);
            current_elf = 0;
            continue;
        }

        current_elf = current_elf + l.parse::<i32>()?;
    }

    elf_calories.sort_by_key(|k| -k);

    println!("max: {:?}", elf_calories[0]);
    println!("top 3: {:?}", elf_calories.iter().take(3).sum::<i32>());

    Ok(())
}
