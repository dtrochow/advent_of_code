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

fn run_path(
    cache: &mut HashMap<String, u64>,
    map: &HashMap<String, Vec<String>>,
    node: &str,
    end: &str,
) -> u64 {
    if cache.contains_key(node) {
        cache[node]
    } else if node == "out" && end != "out" {
        0
    } else if node == end {
        1
    } else {
        let value = map[node].iter().map(|n| run_path(cache, map, n, end)).sum();
        cache.insert(node.into(), value);
        value
    }
}

fn get_answer(map: &HashMap<String, Vec<String>>) -> u64 {
    let mut cache: HashMap<String, u64> = HashMap::new();

    let pairs: Vec<(&str, &str)> = vec![
        ("svr", "fft"),
        ("fft", "dac"),
        ("dac", "out"),
        ("svr", "dac"),
        ("dac", "fft"),
        ("fft", "out"),
    ];

    let mut paths: Vec<u64> = Vec::new();
    for p in pairs {
        cache.clear();
        paths.push(run_path(&mut cache, map, p.0, p.1));
    }

    (paths[0] * paths[1] * paths[2]) + (paths[3] * paths[4] * paths[5])
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let nodes_map = get_nodes_map(lines);

    let answer = get_answer(&nodes_map);
    println!("Number of paths: {}", answer);
}
