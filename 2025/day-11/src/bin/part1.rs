use aoc_lib::read_lines;
use std::collections::HashMap;

fn get_nodes_map(lines: Vec<String>) -> HashMap<String, Vec<String>> {
    lines
        .iter()
        .map(|l| {
            let key: String = l.split(':').next().unwrap().into();
            let value: Vec<String> = l
                .split(':')
                .next_back()
                .unwrap()
                .split_whitespace()
                .map(|e| e.into())
                .collect();
            (key, value)
        })
        .collect()
}

fn run_path(map: &HashMap<String, Vec<String>>, node: &str) -> u64 {
    if node == "out" {
        1
    } else {
        map[node].iter().map(|n| run_path(map, n)).sum()
    }
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let nodes_map = get_nodes_map(lines);
    let number_of_paths = run_path(&nodes_map, "you");

    println!("Number of paths: {}", number_of_paths);
}
