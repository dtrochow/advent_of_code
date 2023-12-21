use std::fs::read_to_string;
use std::time::Instant;

type Pattern = Vec<String>;

enum MirrorType {
    VERTICAL,
    HORIZONTAL
}

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let patterns = get_all_patterns(lines);

    let mut sum: u32 = 0;

    for pattern in &patterns {
        let horizontal_idx = find_mirror(&pattern, MirrorType::HORIZONTAL);
        if horizontal_idx.is_some() {
            sum += 100 * (horizontal_idx.unwrap() + 1);
        }

        let vertical_idx = find_mirror(&pattern, MirrorType::VERTICAL);
        if vertical_idx.is_some() {
            sum += vertical_idx.unwrap() + 1;
        }
    }

    println!("Sum: {}", sum);
    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn find_mirror(pattern: &Pattern, mirror_type: MirrorType) -> Option<u32> {
    match mirror_type {
        MirrorType::VERTICAL => {
            for index in 0..(pattern.first().unwrap().len() - 1) {
                if is_vertical_mirror(pattern, index) {
                    return Some(index as u32);
                }
            }
        },
        MirrorType::HORIZONTAL => {
            for index in 0..(pattern.len() - 1) {
                if is_horizontal_mirror(pattern, index) {
                    return Some(index as u32);
                }
            }
        }
    }
    None
}

fn is_vertical_mirror(pattern: &Pattern, vertical_index: usize) -> bool {
    let length = pattern.first().unwrap().len();
    let lines_to_check: usize = if vertical_index < (length - vertical_index - 1) 
                                 { vertical_index + 1 } else { length - vertical_index - 1 };
    for i in 0..lines_to_check { 
        let left: String = pattern.iter().map(|l| l.chars().nth(vertical_index - i).unwrap()).collect::<String>();
        let right: String = pattern.iter().map(|l| l.chars().nth(vertical_index + i + 1).unwrap()).collect::<String>();
        if left != right {
           return false;
        }
    }
    true
}

fn is_horizontal_mirror(pattern: &Pattern, horizontal_index: usize) -> bool {
    let length = pattern.len();
    let lines_to_check: usize = if horizontal_index < (length - horizontal_index - 1) 
                                 { horizontal_index + 1 } else { length - horizontal_index - 1 };
    for i in 0..lines_to_check {
        if pattern[horizontal_index - i] != pattern[horizontal_index + i + 1] {
           return false;
        }
    }
    true
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
