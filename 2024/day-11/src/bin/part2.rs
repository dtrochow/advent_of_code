use aoc_lib::read_lines;
use std::collections::HashMap;

const BLINKS_COUNT: usize = 75;

type Stone = u64;
type Stones = Vec<Stone>;
type Cache = HashMap<(u64, usize), usize>;

fn get_stones(line: &String) -> Stones {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn has_even_number_of_digits(value: &u64) -> bool {
    value.to_string().len() % 2 == 0
}

fn split_number(value: &u64) -> (u64, u64) {
    let mut value_str = value.to_string();
    let length = value_str.len();
    let (first, second) = value_str.split_at_mut(length / 2);
    (first.parse().unwrap(), second.parse().unwrap())
}

fn blink(stone: Stone, blink_cnt: usize, cache: &mut Cache) -> usize {
    let key = (stone, blink_cnt);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    if blink_cnt == 0 {
        return 1;
    }
    let blink_value;
    if stone == 0 {
        blink_value = blink(1, blink_cnt - 1, cache);
    } else if has_even_number_of_digits(&stone) {
        let split = split_number(&stone);
        blink_value = blink(split.0, blink_cnt - 1, cache) + blink(split.1, blink_cnt - 1, cache);
    } else {
        blink_value = blink(stone * 2024, blink_cnt - 1, cache);
    }
    cache.insert(key, blink_value);
    blink_value
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let stones = get_stones(lines.first().unwrap());

    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();

    let mut stones_cnt = 0;
    for stone in stones {
        stones_cnt += blink(stone, BLINKS_COUNT, &mut cache);
    }

    println!("Number of stones: {}", stones_cnt);
}
