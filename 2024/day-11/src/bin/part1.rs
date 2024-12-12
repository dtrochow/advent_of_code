use aoc_lib::read_lines;

const BLINKS_COUNT: usize = 25;

type Stones = Vec<u64>;

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

fn blink(stones: &mut Stones) -> Stones {
    let mut new_stones: Stones = Vec::new();
    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
        } else if has_even_number_of_digits(stone) {
            let split = split_number(*&stone);
            new_stones.push(split.0);
            new_stones.push(split.1);
        } else {
            new_stones.push(*stone * 2024);
        }
    }
    new_stones
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let mut stones = get_stones(lines.first().unwrap());

    for _ in 0..BLINKS_COUNT {
        stones = blink(&mut stones);
    }

    println!("Number of stones: {}", stones.len());
}
