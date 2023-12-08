#[macro_use]
extern crate maplit;

use lazy_static::lazy_static;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
enum HandType {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPairs = 3,
    Pair = 2,
    Single = 1
}

lazy_static! {
    static ref CARDS: HashMap<char, u32> = hashmap! {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
    };
}

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let mut hands: Vec<(String, u32)> = Vec::with_capacity(lines.len());
    for line in lines {
        let hand: String = line.split(' ').next().unwrap().to_string();
        let bid: u32 = line.split(' ').last().unwrap().parse().unwrap();
        hands.push((hand, bid));
    }

    hands.sort_by(|a, b| compare_two_hands(&a.0, &b.0));

    let mut total_winnings: u64 = 0;
    for (index, hand) in hands.iter().enumerate() {
        let bid = hand.1;
        let rank: u32 = index as u32 + 1;
        total_winnings += (rank * bid) as u64;
    }
    println!("Total winnings: {}", total_winnings);

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn find_type(hand: &String) -> HandType {
    if one_of(5, hand, true)  { return HandType::Five;      }
    if one_of(4, hand, true)  { return HandType::Four;      }
    if full_house(hand)       { return HandType::FullHouse; }
    if one_of(3, hand, true)  { return HandType::Three;     }
    if two_pairs(hand, true)  { return HandType::TwoPairs;  }
    if one_of(2, hand, true)  { return HandType::Pair;      }

    HandType::Single
}

fn one_of(how_many: u32, hand: &String, check_joker: bool) -> bool {
    let mut card_values: HashMap<u32, u32> = HashMap::new();
    for ch in hand.chars() {
        card_values.entry(CARDS[&ch])
                   .and_modify(|existing_val| *existing_val += 1)
                   .or_insert(1);
    }

    let mut max_count: u32 = 0;
    for (card_val, count) in card_values.clone().into_iter() {
        if (count > max_count) && (card_val != CARDS[&'J']) { max_count = count; }
        if count == how_many { return true; }
    }

    let jokers_count = count_jokers(hand);
    if ((how_many - max_count) <= jokers_count) && check_joker { return true; }

    false
}

fn count_jokers(hand: &String) -> u32 {
    hand.chars().filter(|&ch| ch == 'J').count().try_into().unwrap()
}

fn full_house(hand: &String) -> bool {
    let jokers_count = count_jokers(hand);
    if (one_of(3, hand, false) && one_of(2, hand, false)) || 
       (one_of(3, hand, false) && (jokers_count >= 1)) ||
       (two_pairs(hand, false) && (jokers_count >= 1)) || 
       (jokers_count >= 4)
    {
        true
    } else {
        false
    }
}

fn two_pairs(hand: &String, check_joker: bool) -> bool {
    let mut card_values: HashMap<u32, u32> = HashMap::new();
    for ch in hand.chars() {
        card_values.entry(CARDS[&ch])
                   .and_modify(|existing_val| *existing_val += 1)
                   .or_insert(1);
    }

    let pairs_count: u32 = card_values.values().filter(|&val| *val == 2).count().try_into().unwrap();

    let jokers_count = count_jokers(hand);
    if (pairs_count == 2) || ((pairs_count == 1) && (jokers_count >= 1) && check_joker) {
        return true;
    }

    false
}

fn greater_first_card(a: &String, b: &String) -> Ordering {
    for (a_val, b_val) in a.chars().into_iter().zip(b.chars().into_iter()) {
        if a_val != b_val {
            if CARDS[&a_val] < CARDS[&b_val] {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
    }
    Ordering::Equal
}

// Check if A is LESS or GREATER than B
fn compare_two_hands(a: &String, b: &String) -> Ordering {
    let a_type = find_type(a);
    let b_type = find_type(b);

    if a_type > b_type {
        return Ordering::Greater;
    } else if a_type < b_type {
        return Ordering::Less;
    } else {
        greater_first_card(a, b)
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
