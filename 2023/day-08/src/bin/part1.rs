use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let instructions: Vec<char> = lines.first().unwrap().chars().collect();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for node in lines.iter().skip(2) {
        let node_id = node.split('=').next().unwrap().trim().to_string();
        let left_connection = node
            .split('(')
            .last()
            .unwrap()
            .split(',')
            .next()
            .unwrap()
            .trim()
            .to_string();
        let right_connection = node
            .split(')')
            .next()
            .unwrap()
            .split(',')
            .last()
            .unwrap()
            .trim()
            .to_string();
        map.insert(node_id, (left_connection, right_connection));
    }

    let mut current_node: String = "AAA".to_string();
    let mut steps_count: u32 = 0;
    while current_node != *"ZZZ" {
        let instr_index: usize = (steps_count as usize) % instructions.len();
        current_node = get_next_node(&current_node, instructions[instr_index], &map);
        steps_count += 1;
    }
    println!("Total number of steps: {}", steps_count);

    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

fn get_next_node(
    current_node: &String,
    instruction: char,
    map: &HashMap<String, (String, String)>,
) -> String {
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
