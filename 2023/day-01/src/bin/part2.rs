#[macro_use]
extern crate maplit;

use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let mut sum: u32 = 0;
    for line in lines {
        sum += find_number(&line);
    }
    println!("The sum is: {}", sum);
}

fn find_number_in_string(characters: &str) -> Option<u32> {
    let string_digit_map: HashMap<&str, u32> = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    };

    for (key, value) in &string_digit_map {
        if characters.contains(key) {
            return Some(*value);
        }
    }

    for ch in characters.chars() {
        if ch.is_digit(10)
        {
            return Some(ch.to_digit(10).unwrap());
        }
    }

    None
}

fn prepend_string(string: String, c: char) -> String {
    let mut prepended_string: String = String::new();
    prepended_string.push(c);
    prepended_string.push_str(&string);
    prepended_string
}

fn find_number(obfuscated_str: &str) -> u32 {
    let mut first_digit = 0;
    let mut second_digit = 0;

    // Find first to the left
    let mut left_string: String = String::new();
    for character in obfuscated_str.chars() {
        left_string.push(character);
        let result = find_number_in_string(&left_string);
        match result {
            Some(value) => { 
                first_digit = value;
                break;
            },
            None => (),
        }
    }

    // Find first to the right
    let mut right_string: String = String::new();
    for character in obfuscated_str.chars().rev() {
        right_string = prepend_string(right_string, character);
        let result = find_number_in_string(&right_string);
        match result {
            Some(value) => { 
                second_digit = value;
                break;
            },
            None => (),
        }
    }

    (10 * first_digit) + second_digit
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
