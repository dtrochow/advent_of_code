use std::fs::read_to_string;

#[derive(Debug, Clone, Default)]
struct BitCount {
    ones: u64,
    zeros: u64,
}

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let oxygen_generator_rating = get_oxygen_generator_rating(&lines);
    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    let co2_scrubber_rating = get_co2_scrubber_rating(&lines);
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);

    println!(
        "Life support rating: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}

fn filter_numbers_with_bit_on_pos(numbers: &Vec<String>, pos: usize, value: char) -> Vec<String> {
    let mut filtered_vec: Vec<String> = Vec::<String>::new();
    for number in numbers {
        if number.chars().nth(pos).unwrap() == value {
            filtered_vec.push(number.to_string());
        }
    }
    filtered_vec
}

fn get_oxygen_generator_rating(lines: &[String]) -> u32 {
    fn get_most_common(bit_count: BitCount) -> char {
        if bit_count.ones >= bit_count.zeros {
            '1'
        } else {
            '0'
        }
    }
    get_rating(lines, &get_most_common)
}

fn get_co2_scrubber_rating(lines: &[String]) -> u32 {
    fn get_least_common(bit_count: BitCount) -> char {
        if bit_count.ones >= bit_count.zeros {
            '0'
        } else {
            '1'
        }
    }
    get_rating(lines, &get_least_common)
}

fn get_rating(lines: &[String], f: &dyn Fn(BitCount) -> char) -> u32 {
    let mut filtered: Vec<String> = lines.to_owned();
    let mut pos: usize = 0;
    while filtered.len() != 1 {
        let bit_count: BitCount = get_bits_count_on_position(&filtered, pos);
        filtered = filter_numbers_with_bit_on_pos(&filtered, pos, f(bit_count));
        pos += 1;
    }
    match u32::from_str_radix(filtered.first().unwrap(), 2) {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse gamma rate value!");
            0
        }
    }
}

fn get_bits_count_on_position(lines: &Vec<String>, pos: usize) -> BitCount {
    let mut bit_count: BitCount = BitCount::default();
    for line in lines {
        match line.chars().nth(pos).unwrap() {
            '0' => bit_count.zeros += 1,
            '1' => bit_count.ones += 1,
            _ => println!("Error - Unexpected bit value!"),
        }
    }
    bit_count
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
