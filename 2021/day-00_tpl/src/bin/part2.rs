use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    println!("Lines: {:?}", lines);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
