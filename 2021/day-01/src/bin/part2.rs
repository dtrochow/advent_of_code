use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let increased: u64 = get_increased_count(&lines);
    println!("Increased count: {}", increased);
}

fn get_three_sum(lines: &[String], index: usize) -> u64 {
    lines[index..index + 3]
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .sum()
}

fn get_increased_count(lines: &[String]) -> u64 {
    let mut increased_count: u64 = 0;
    let mut previous: Option<u64> = None;
    for i in 0..lines.len() - 2 {
        let current_value: u64 = get_three_sum(lines, i);
        if let Some(prev) = previous {
            if current_value > prev {
                increased_count += 1
            }
        }
        previous = Some(current_value);
    }
    increased_count
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
