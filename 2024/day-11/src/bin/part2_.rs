use aoc_lib::read_lines;
use cached::proc_macro::cached;

const BLINKS_COUNT: usize = 75;

type Stone = u64;
type Stones = Vec<Stone>;

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

#[cached]
fn blink(stone: Stone, blink_cnt: usize) -> usize {
    if blink_cnt == 0 {
        return 1;
    }
    if stone == 0 {
        return blink(1, blink_cnt - 1);
    } else if has_even_number_of_digits(&stone) {
        let split = split_number(&stone);
        return blink(split.0, blink_cnt - 1) + blink(split.1, blink_cnt - 1);
    } else {
        return blink(stone * 2024, blink_cnt - 1);
    }
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let stones = get_stones(lines.first().unwrap());

    let mut stones_cnt = 0;
    for stone in stones {
        stones_cnt += blink(stone, BLINKS_COUNT);
    }

    println!("Number of stones: {}", stones_cnt);
}
