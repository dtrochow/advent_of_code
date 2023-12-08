use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let instructions: Vec<char> = lines.first().unwrap().chars().collect();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for node in lines.iter().skip(2) {
        let node_id = node.split('=').next().unwrap().trim().to_string();
        let left_connection = node.split('(').last().unwrap().split(',').next().unwrap().trim().to_string();
        let right_connection = node.split(')').next().unwrap().split(',').last().unwrap().trim().to_string();
        map.insert(node_id, (left_connection, right_connection));
    }

    let mut current_nodes: Vec<String> = find_all_start_nodes(map.keys().cloned().collect());
    current_nodes.sort();

    let mut all_steps_count: Vec<u64> = Vec::new();
    for node in current_nodes {
        let mut current_node = node;
        let mut steps_count: u64 = 0;
        while !current_node.ends_with('Z') {
            let instr_index: usize = ((steps_count as usize) % instructions.len()) as usize;
            current_node = get_next_node(&current_node, instructions[instr_index], &map);
            steps_count += 1;
        }
        all_steps_count.push(steps_count);
    }

    // Can use the LCM, because the lowest common multiplier equals to
    // number of steps needed to get from start to the end
    let mut all_steps: u64 = 1;
    for steps in &all_steps_count {
        all_steps = lcm(all_steps, *steps);
    }
    println!("Steps counts: {:?}", all_steps_count);
    println!("All steps to the end: {}", all_steps);

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn find_all_start_nodes(nodes: Vec<String>) -> Vec<String> {
    let mut start_nodes: Vec<String> = Vec::new();
    for node in nodes {
        if node.ends_with('A') {
            start_nodes.push(node);
        }
    }
    start_nodes
}

fn get_next_node(current_node: &String, instruction: char, map: &HashMap<String, (String, String)>) -> String {
    let node_connections = map.get(current_node).unwrap();
    if instruction == 'L' {
        node_connections.0.clone()
    } else if instruction == 'R' {
        node_connections.1.clone()
    } else {
        String::new()
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
