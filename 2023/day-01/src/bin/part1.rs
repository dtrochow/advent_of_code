use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let mut sum: u32 = 0;
    for line in lines {
        sum += find_number(&line);
    }
    println!("The sum is: {}", sum);
}

fn find_number(obfuscated_str: &str) -> u32 {
    let mut digit_vec: Vec<u32> = Vec::new();
    for ch in obfuscated_str.chars() {
        if ch.is_ascii_digit() {
            digit_vec.push(ch.to_digit(10).unwrap());
        }
    }

    (10 * digit_vec.first().unwrap()) + digit_vec.last().unwrap()
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
