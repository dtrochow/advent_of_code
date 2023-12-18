use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let line: String = read_lines("./src/bin/input1.txt").first().unwrap().to_string();
    let steps: Vec<String> = get_all_steps(line);

    let mut sum: u64 = 0;
    for step in steps {
        sum += calculate_hash_value(step) as u64;
    }
    println!("Sum: {}", sum);
    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn calculate_hash_value(step: String) -> u8 {
    let mut hash: u32 = 0;
    for ch in step.chars() {
        let ascii_value: u8 = ch as u8;
        hash = ((hash + ascii_value as u32) * 17) % 256;
    }
    hash as u8
}

fn get_all_steps(line: String) -> Vec<String> {
    line.split(',').map(|s| s.parse().expect("Parsing error")).collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
