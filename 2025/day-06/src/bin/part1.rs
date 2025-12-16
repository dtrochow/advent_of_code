use aoc_lib::read_lines;

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
    Invalid,
}

type Numbers = Vec<Vec<u64>>;
type Operations = Vec<Operation>;

fn get_numbers(lines: &[String]) -> Numbers {
    let mut numbers: Numbers = Vec::new();
    for line in lines.iter().take(lines.len() - 1) {
        numbers.push(
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        );
    }
    numbers
}

fn get_operations(lines: &[String]) -> Operations {
    lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|o| match o {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => Operation::Invalid,
        })
        .collect()
}

fn perform_operations(numbers: Numbers, operations: Operations) -> u64 {
    let mut sum: u64 = 0;
    for (i, op) in operations.iter().enumerate() {
        let nums: Vec<u64> = numbers.iter().map(|l| l[i]).collect();
        match op {
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

    let numbers = get_numbers(&lines);
    let operations = get_operations(&lines);

    let result = perform_operations(numbers, operations);
    println!("Result: {}", result);
}
