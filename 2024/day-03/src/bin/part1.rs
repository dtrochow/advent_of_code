use aoc_lib::read_lines;
use regex::Regex;

fn get_valid_factor_pairs(lines: Vec<String>) -> Vec<(i32, i32)> {
    let joined_list: String = lines.join("\n");

    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    re.captures_iter(&joined_list)
        .map(|cap| {
            let num1 = cap[1].parse::<i32>().unwrap();
            let num2 = cap[2].parse::<i32>().unwrap();
            (num1, num2)
        })
        .collect()
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let factor_pairs = get_valid_factor_pairs(lines);

    let mut result = 0;
    for pair in factor_pairs {
        result += pair.0 * pair.1;
    }

    println!("Result: {}", result);
}
