use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let repetitive_priorities_sum = get_repetitive_items_priorities_sum(lines);
    println!("Sum of priorities: {}", repetitive_priorities_sum);
}

fn get_repetitive_items_priorities_sum(lines: Vec<String>) -> u32 {
    let mut repetitive_item_priorities_sum: u32 = 0;

    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut temp_group: Vec<String> = Vec::new();
    for rucksack in lines {
        temp_group.push(rucksack);
        if temp_group.len() >= 3 {
            groups.push(temp_group.clone());
            temp_group = Vec::new();
        }
    }
    for group in groups {
        repetitive_item_priorities_sum += get_group_badge_type_priority(group);
    }
    repetitive_item_priorities_sum
}

fn get_group_badge_type_priority(group: Vec<String>) -> u32 {
    let mut rucksacks: Vec<HashSet<char>> = Vec::new();
    for backpack in &group {
        rucksacks.push(HashSet::from_iter(backpack.chars()));
    }

    let (intersection, others) = rucksacks.split_at_mut(1);
    let badge = &mut intersection[0];
    for rucksack in others {
        badge.retain(|e| rucksack.contains(e));
    }

    let badge = *badge.iter().next().unwrap();
    if badge.is_lowercase() {
        (badge as u32) - ('`' as u32)
    } else {
        (badge as u32) - ('@' as u32) + 26
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
