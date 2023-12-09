use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let mut extrapolated_values_sum: i64 = 0;

    for line in lines {
        let values: Vec<i64> = line.split_whitespace().map(|v| v.parse().expect("Parsing error")).collect();
        extrapolated_values_sum += find_extrapolation(values);
    }

    println!("Sum of all extrapolations: {}", extrapolated_values_sum);

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn find_extrapolation(values: Vec<i64>) -> i64 {
    let mut ext_vectors: Vec<Vec<i64>> = Vec::new();
    ext_vectors.push(values);

    while !ext_vectors.last().unwrap().iter().all(|&v| v == 0) {
        let current_vector = ext_vectors.last().unwrap();
        let mut next_vec: Vec<i64> = Vec::new();
        for i in 0..(current_vector.len() - 1) {
            next_vec.push(current_vector[i+1] - current_vector[i]);
        }
        ext_vectors.push(next_vec);
    }

    let mut ext_value: i64 = 0;
    for vec in ext_vectors.iter().rev().skip(1) {
        ext_value = ext_value + vec.last().unwrap();
    }
    ext_value
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
