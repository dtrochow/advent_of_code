use aoc_lib::read_lines;

#[derive(Debug)]
struct IdRange {
    start: u64,
    end: u64,
}

fn get_id_ranges(line: &str) -> Vec<IdRange> {
    line.split(',')
        .map(|r| {
            let start: u64 = r.split('-').next().unwrap().parse().unwrap();
            let end: u64 = r.split('-').next_back().unwrap().parse().unwrap();
            IdRange { start, end }
        })
        .collect()
}

fn is_id_invalid(id: u64) -> bool {
    let str = id.to_string();
    let size = str.len();
    let digits: Vec<char> = str.chars().collect();

    for sub_len in 1..=(size / 2) {
        let number_of_parts = size / sub_len;
        if size != (number_of_parts * sub_len) {
            continue;
        }

        let mut is_invalid: bool = true;
        for i in 0..sub_len {
            let mut prev_digit = digits[i];
            for y in 1..number_of_parts {
                if digits[i + (y * sub_len)] != prev_digit {
                    is_invalid = false;
                }
                prev_digit = digits[i + (y * sub_len)];
            }
        }
        if is_invalid {
            return true;
        }
    }

    false
}

fn get_invalid_ids_sum(range: IdRange) -> u64 {
    let mut sum: u64 = 0;
    for id in range.start..=range.end {
        if is_id_invalid(id) {
            sum += id;
        }
    }
    sum
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let id_ranges: Vec<IdRange> = get_id_ranges(lines.first().unwrap());

    let mut sum: u64 = 0;
    for range in id_ranges {
        sum += get_invalid_ids_sum(range);
    }

    println!("Invalid ID ranges sum: {}", sum);
}
