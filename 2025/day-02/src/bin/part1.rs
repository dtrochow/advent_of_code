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
    let mid = str.len() / 2;
    let (left, right) = str.split_at(mid);

    left == right
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
