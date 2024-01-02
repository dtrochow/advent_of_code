use std::fs::read_to_string;
use std::time::Instant;

type Pattern = Vec<String>;

enum MirrorType {
    Vertical,
    Horizontal,
}

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let patterns = get_all_patterns(lines);

    let mut sum: u32 = 0;
    for pattern in &patterns {
        let horizontal_idx = find_smudge_mirror(pattern, MirrorType::Horizontal);
        if let Some(id) = horizontal_idx {
            sum += 100 * (id + 1);
        }

        let vertical_idx = find_smudge_mirror(pattern, MirrorType::Vertical);
        if let Some(id) = vertical_idx {
            sum += id + 1;
        }
    }

    println!("Sum: {}", sum);
    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

fn find_smudge_mirror(pattern: &Pattern, mirror_type: MirrorType) -> Option<u32> {
    match mirror_type {
        MirrorType::Vertical => {
            for index in 0..(pattern.first().unwrap().len() - 1) {
                if is_vertical_smudge_mirror(pattern, index) {
                    return Some(index as u32);
                }
            }
        }
        MirrorType::Horizontal => {
            for index in 0..(pattern.len() - 1) {
                if is_horizontal_smudge_mirror(pattern, index) {
                    return Some(index as u32);
                }
            }
        }
    }
    None
}

fn compare_string_differences(string1: &str, string2: &str) -> u32 {
    let size: usize = string1.len();
    let mut differences: u32 = 0;
    for i in 0..size {
        if string1.chars().nth(i).unwrap() != string2.chars().nth(i).unwrap() {
            differences += 1;
        }
    }
    differences
}

fn is_vertical_smudge_mirror(pattern: &Pattern, vertical_index: usize) -> bool {
    let length = pattern.first().unwrap().len();
    let lines_to_check: usize = if vertical_index < (length - vertical_index - 1) {
        vertical_index + 1
    } else {
        length - vertical_index - 1
    };
    let mut smudge_count: u32 = 0;
    for i in 0..lines_to_check {
        let left: String = pattern
            .iter()
            .map(|l| l.chars().nth(vertical_index - i).unwrap())
            .collect::<String>();
        let right: String = pattern
            .iter()
            .map(|l| l.chars().nth(vertical_index + i + 1).unwrap())
            .collect::<String>();
        smudge_count += compare_string_differences(&left, &right);
    }
    smudge_count == 1
}

fn is_horizontal_smudge_mirror(pattern: &Pattern, horizontal_index: usize) -> bool {
    let length = pattern.len();
    let lines_to_check: usize = if horizontal_index < (length - horizontal_index - 1) {
        horizontal_index + 1
    } else {
        length - horizontal_index - 1
    };
    let mut smudge_count: u32 = 0;
    for i in 0..lines_to_check {
        smudge_count += compare_string_differences(
            &pattern[horizontal_index - i],
            &pattern[horizontal_index + i + 1],
        );
    }
    smudge_count == 1
}

fn get_all_patterns(lines: Vec<String>) -> Vec<Pattern> {
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut pattern: Pattern = Vec::new();
    for line in lines {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = Vec::new();
            continue;
        }
        pattern.push(line);
    }
    patterns.push(pattern);
    patterns
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
