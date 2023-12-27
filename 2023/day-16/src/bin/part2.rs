use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let line: String = read_lines("./src/bin/input1.txt").first().unwrap().to_string();

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
