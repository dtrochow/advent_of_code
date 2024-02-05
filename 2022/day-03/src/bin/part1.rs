use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let repetitive_priorities_sum = get_repetitive_items_priorities_sum(lines);
    println!("Sum of priorities: {}", repetitive_priorities_sum);
}

fn get_repetitive_items_priorities_sum(lines: Vec<String>) -> u32 {
    let mut repetitive_item_priorities_sum: u32 = 0;

    for line in lines {
        repetitive_item_priorities_sum += get_rucksack_repetitive_item_priorities_sum(line);
    }

    repetitive_item_priorities_sum
}

fn get_rucksack_repetitive_item_priorities_sum(line: String) -> u32 {
    let mut repetitive_item_priorities_sum: u32 = 0;

    // Cut the string in half
    let first_compartment: &str = &line[..(line.len() / 2)];
    let second_compartment: &str = &line[(line.len() / 2)..];

    // Get all repetitive items
    let mut repetitive_items: Vec<char> = Vec::new();
    for ch in first_compartment.chars() {
        if second_compartment.find(ch).is_some() {
            repetitive_items.push(ch);
        }
    }
    repetitive_items.dedup();

    // Convert the tags to priorities
    for ch in repetitive_items {
        if ch.is_lowercase() {
            repetitive_item_priorities_sum += (ch as u32) - ('`' as u32);
        } else {
            repetitive_item_priorities_sum += (ch as u32) - ('@' as u32) + 26;
        }
    }

    repetitive_item_priorities_sum
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
