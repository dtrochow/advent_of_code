use aoc_lib::{find_index, read_lines};

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

fn get_ranges(lines: &[String]) -> Vec<Range> {
    let end_index = find_index(lines, &"".to_string()).unwrap();
    lines[..end_index]
        .iter()
        .map(|l| {
            let start: u64 = l.split('-').next().unwrap().parse().unwrap();
            let end: u64 = l.split('-').next_back().unwrap().parse().unwrap();
            Range { start, end }
        })
        .collect()
}

fn merge_ranges(ranges: &mut Vec<Range>, first_index: usize, second_index: usize) {
    let new_end = ranges[second_index].end;
    ranges.remove(second_index);
    if new_end > ranges[first_index].end {
        ranges[first_index].end = new_end;
    }
}

fn get_number_of_fresh_ingredients(ranges: Vec<Range>) -> u64 {
    ranges.iter().map(|r| r.end - (r.start + 1)).sum()
}

fn get_fresh_ingredients(ranges: Vec<Range>) -> u64 {
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_by_key(|k| k.start);

    let mut prev_range_index = 0;
    let mut index = 1;
    loop {
        if index >= sorted_ranges.len() {
            break;
        }
        let prev_range = sorted_ranges[prev_range_index];
        let curr_range = sorted_ranges[index];
        if curr_range.start <= prev_range.end {
            merge_ranges(&mut sorted_ranges, prev_range_index, index);
        } else {
            prev_range_index = index;
            index += 1;
        }
    }

    get_number_of_fresh_ingredients(sorted_ranges)
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let ranges = get_ranges(&lines);
    let number_of_fresh_ids = get_fresh_ingredients(ranges);
    println!("Number of fresh ingredients: {}", number_of_fresh_ids);
}
