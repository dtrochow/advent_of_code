use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut cards_num: Vec<u32> = Vec::with_capacity(lines.len());
    // Initialize all cards
    cards_num.resize(lines.len(), 1);

    for (id, line) in lines.iter().enumerate() {
        let matching_values = find_cards_matches(line);
        let current_cards_num = cards_num[id];
        for i in 1..(matching_values+1) {
            let index: usize = i as usize;
            let card_id: usize = id + index;
            cards_num[card_id] += current_cards_num;
        }
    }
    let sum: u32 = cards_num.iter().sum();
    println!("The sum is: {}", sum);
}

fn find_cards_matches(card_info: &String) -> u32 {
    let card_info_without_id: String = card_info.split(':').last().unwrap().to_string();
    let winning_numbers: Vec<u32> = get_winning_numbers(&card_info_without_id);
    let card_numbers: Vec<u32> = get_card_numbers(&card_info_without_id);

    let mut value_matches: u32 = 0;

    for win_num in winning_numbers {
        if card_numbers.contains(&win_num) {
            value_matches += 1;
        }
    }
    value_matches
}

fn get_winning_numbers(card_info_without_id: &String) -> Vec<u32> {
    let winning_num_str: String = card_info_without_id.split("|").next().unwrap().to_string();
    let winning_numbers: Vec<u32> = winning_num_str.split_whitespace()
                                                   .map(|s| s.parse().expect("Parsing error"))
                                                   .collect();
    winning_numbers
}

fn get_card_numbers(card_info_without_id: &String) -> Vec<u32> {
    let card_numbers_str: String = card_info_without_id.split("|").last().unwrap().to_string();
    let card_numbers: Vec<u32> = card_numbers_str.split_whitespace()
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
