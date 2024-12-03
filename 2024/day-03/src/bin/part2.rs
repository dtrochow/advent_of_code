use aoc_lib::read_lines;
use regex::Regex;

const STOP: &str = "don't()";
const START: &str = "do()";

fn get_all_valid_commands(lines: Vec<String>) -> Vec<String> {
    let joined_list: String = lines.join("\n");

    let pattern = r"(mul\(\d+,\d+\)|do\(\)|don't\(\))";
    let re = Regex::new(pattern).unwrap();

    re.find_iter(&joined_list)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

fn get_mul_factors(mul_command: String) -> (i64, i64) {
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

    let caps = re.captures(&mul_command).unwrap();
    let a: i64 = caps[1].parse().unwrap();
    let b: i64 = caps[2].parse().unwrap();

    (a, b)
}

fn process_commands(commands: Vec<String>) -> i64 {
    let mut is_enabled: bool = true;
    let mut result = 0;

    for command in commands {
        if command == STOP {
            is_enabled = false;
            continue;
        } else if command == START {
            is_enabled = true;
            continue;
        }

        if is_enabled {
            let mul_factors = get_mul_factors(command);
            result += mul_factors.0 * mul_factors.1;
        }
    }

    result
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let valid_commands = get_all_valid_commands(lines);
    let result = process_commands(valid_commands);

    println!("Result: {}", result);
}
