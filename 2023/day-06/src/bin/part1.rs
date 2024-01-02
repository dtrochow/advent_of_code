use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let times: Vec<u32> = lines[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Parsing error"))
        .collect();
    let record_distances: Vec<u32> = lines[1]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Parsing error"))
        .collect();

    let mut number_of_ways: Vec<u32> = Vec::new();
    number_of_ways.resize(times.len(), 0);

    for (index, time) in times.iter().enumerate() {
        for s in 1..*time {
            let distance = s * (time - s);
            if distance > record_distances[index] {
                number_of_ways[index] += 1;
            }
        }
    }

    println!(
        "The product of numbers of ways is: {}",
        number_of_ways.iter().product::<u32>()
    );

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
