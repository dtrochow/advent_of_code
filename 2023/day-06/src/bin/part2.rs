use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let time: u64 = lines[0]
        .trim()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .expect("Parsing error");
    let record_distance: u64 = lines[1]
        .trim()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .expect("Parsing error");

    let mut number_of_ways: u32 = 0;

    for s in 1..time {
        let distance = s * (time - s);
        if distance > record_distance {
            number_of_ways += 1;
        }
    }

    println!("The numbers of ways is: {}", number_of_ways);

    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
