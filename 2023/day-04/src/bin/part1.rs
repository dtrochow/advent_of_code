use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut sum: u32 = 0;
    for line in lines {
        sum += find_cards_value(line);
    }
    println!("The sum is: {}", sum);
}

fn find_cards_value(card_info: String) -> u32 {
    let card_info_without_id: String = card_info.split(':').last().unwrap().to_string();
    let winning_numbers: Vec<u32> = get_winning_numbers(&card_info_without_id);
    let card_numbers: Vec<u32> = get_card_numbers(&card_info_without_id);

    let mut card_value: u32 = 0;

    for win_num in winning_numbers {
        if card_numbers.contains(&win_num) {
            if card_value == 0 {
                card_value = 1;
            } else {
                card_value <<= 1;
            }
        }
    }
    card_value
}

fn get_winning_numbers(card_info_without_id: &str) -> Vec<u32> {
    let winning_num_str: String = card_info_without_id.split('|').next().unwrap().to_string();
    let winning_numbers: Vec<u32> = winning_num_str
        .split_whitespace()
        .map(|s| s.parse().expect("Parsing error"))
        .collect();
    winning_numbers
}

fn get_card_numbers(card_info_without_id: &str) -> Vec<u32> {
    let card_numbers_str: String = card_info_without_id.split('|').last().unwrap().to_string();
    let card_numbers: Vec<u32> = card_numbers_str
        .split_whitespace()
        .map(|s| s.parse().expect("Parsing error"))
        .collect();
    card_numbers
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
