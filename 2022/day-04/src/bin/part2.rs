use std::fs::read_to_string;

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32, // inclusively
}

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let elf_pairs: Vec<(Range, Range)> = get_elf_pairs(lines);
    let overlapping_count: u32 = count_overlapping_elf_pairs(elf_pairs);
    println!("Overlapping count: {}", overlapping_count);
}

fn count_overlapping_elf_pairs(elf_pairs: Vec<(Range, Range)>) -> u32 {
    let mut overlapping_count: u32 = 0;
    for pair in elf_pairs {
        if is_pair_overlapping(pair) {
            overlapping_count += 1;
        }
    }
    overlapping_count
}

fn is_pair_overlapping(pair: (Range, Range)) -> bool {
    ((pair.0.start >= pair.1.start) && (pair.0.start <= pair.1.end))
        || ((pair.1.start >= pair.0.start) && (pair.1.start <= pair.0.end))
        || ((pair.0.end >= pair.1.start) && (pair.0.end <= pair.1.end))
        || ((pair.1.end >= pair.0.start) && (pair.1.end <= pair.0.end))
}

fn get_elf_pairs(lines: Vec<String>) -> Vec<(Range, Range)> {
    let mut elf_pairs: Vec<(Range, Range)> = Vec::new();
    for line in lines {
        elf_pairs.push(get_elf_pair(line));
    }
    elf_pairs
}

fn get_elf_pair(line: String) -> (Range, Range) {
    let first = line.split(',').next().unwrap();
    let second = line.split(',').last().unwrap();
    (
        Range {
            start: first.split('-').next().unwrap().parse().unwrap(),
            end: first.split('-').last().unwrap().parse().unwrap(),
        },
        Range {
            start: second.split('-').next().unwrap().parse().unwrap(),
            end: second.split('-').last().unwrap().parse().unwrap(),
        },
    )
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
