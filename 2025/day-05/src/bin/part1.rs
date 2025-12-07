use aoc_lib::{find_index, read_lines};

type Id = u64;

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

fn get_ids(lines: &[String]) -> Vec<Id> {
    let start_index = find_index(lines, &"".to_string()).unwrap();
    lines[start_index + 1..]
        .iter()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn is_in_range(id: Id, range: Range) -> bool {
    id >= range.start && id <= range.end
}

fn is_in_any_range(id: Id, ranges: Vec<Range>) -> bool {
    ranges.iter().any(|r| is_in_range(id, *r))
}

fn get_fresh_ingredients(ranges: Vec<Range>, ids: Vec<Id>) -> usize {
    ids.iter()
        .filter(|&id| is_in_any_range(*id, ranges.clone()))
        .count()
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let ranges = get_ranges(&lines);
    let ids = get_ids(&lines);

    let number_of_fresh_ids = get_fresh_ingredients(ranges, ids);
    println!("Number of fresh ingredients: {}", number_of_fresh_ids);
}
