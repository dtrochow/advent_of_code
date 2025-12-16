use aoc_lib::read_lines;

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
    Invalid,
}

type Numbers = Vec<Vec<u64>>;
type Operations = Vec<(Operation, usize)>;

fn get_numbers(lines: &[String], operations: &Operations) -> Numbers {
    let mut numbers: Numbers = Vec::new();
    let mut chunks: Vec<Vec<String>> = Vec::new();
    for line in lines.iter().take(lines.len() - 1) {
        let current_line: Vec<char> = line.chars().collect();
        let mut current_chunks: Vec<String> = Vec::new();
        for (_, chunk_len) in operations {
            let mut current_pos: usize = current_chunks.iter().map(|c| c.len()).sum();
            if current_pos != 0 {
                current_pos += current_chunks.len();
            }
            let chunk = &current_line[current_pos..current_pos + chunk_len];
            current_chunks.push(chunk.iter().collect());
        }
        chunks.push(current_chunks);
    }

    for (i, chunk) in chunks.first().unwrap().iter().enumerate() {
        let mut numbers_set: Vec<u64> = Vec::new();
        for j in 0..chunk.len() {
            let mut number: Vec<char> = Vec::new();
            for chunk in &chunks {
                number.push(chunk[i].chars().collect::<Vec<char>>()[j]);
            }
            numbers_set.push(
                number
                    .into_iter()
                    .collect::<String>()
                    .trim()
                    .parse()
                    .unwrap(),
            );
        }
        numbers.push(numbers_set);
    }

    numbers
}

fn get_operations(lines: &[String]) -> Operations {
    let mut ops: Operations = Vec::new();
    for (i, ch) in lines.last().unwrap().chars().rev().enumerate() {
        if ch != ' ' {
            let size = if ops.is_empty() {
                i + 1
            } else {
                i - ops.iter().map(|o| o.1).sum::<usize>() - ops.len() + 1
            };
            ops.push((
                match ch {
                    '*' => Operation::Multiply,
                    '+' => Operation::Add,
                    _ => Operation::Invalid,
                },
                size,
            ));
        }
    }
    ops.into_iter().rev().collect()
}

fn perform_operations(numbers: Numbers, operations: Operations) -> u64 {
    let mut sum: u64 = 0;
    for (i, op) in operations.iter().enumerate() {
        let nums = &numbers[i];
        match op.0 {
            Operation::Add => {
                sum += nums.iter().sum::<u64>();
            }
            Operation::Multiply => {
                sum += nums.iter().product::<u64>();
            }
            Operation::Invalid => {}
        }
    }
    sum
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let operations = get_operations(&lines);
    let numbers = get_numbers(&lines, &operations);

    let result = perform_operations(numbers, operations);
    println!("Result: {}", result);
}
