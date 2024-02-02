use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut elf_calories = count_all_calories(lines);
    elf_calories.sort();

    println!("The three top calories count: {}",
             elf_calories[elf_calories.len() - 1] +
             elf_calories[elf_calories.len() - 2] +
             elf_calories[elf_calories.len() - 3],
             );
}

fn count_all_calories(lines: Vec<String>) -> Vec<u32> {
    let mut calories_counts: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    for line in lines {
        if line.is_empty() {
            calories_counts.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<u32>().unwrap();
    }
    calories_counts
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
