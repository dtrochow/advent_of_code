use std::fs::read_to_string;
use std::time::Instant;

struct Range {
    dst_start: i64,
    src_start: i64,
    length: i64
}

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let seeds = get_seeds(&lines[0]);
    let all_maps = get_all_maps_vector(&lines);

    let mut lowest_location: i64 = i64::MAX;
    for seed in seeds {
        let mut location: i64 = seed;
        for map in &all_maps {
            location = find_mapping_value(&map, location);
        }
        if location < lowest_location {
            lowest_location = location;
        }
    }
    println!("The lowest location is: {}", lowest_location);

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn find_mapping_value(map: &Vec<Range>, value: i64) -> i64 {
    for range in map {
        let src_range = range.src_start..(range.src_start+range.length);
        if src_range.contains(&value) {
            let pos_diff: i64 = range.dst_start - range.src_start;
            return value + pos_diff;
        }
    }
    value
}

fn get_seeds(almanc: &String) -> Vec<i64> {
    almanc.split(':').last().unwrap().split_whitespace()
                                     .map(|s| s.parse().expect("Parsing error"))
                                     .collect()
}

fn get_all_maps_vector(lines: &Vec<String>) -> Vec<Vec<Range>> {
    let map_positions: Vec<(usize, usize)> = vec![
        (3, 11),
        (14, 39),
        (42, 70),
        (73, 93),
        (96, 114),
        (117, 159),
        (162, 188)
    ];

    let mut all_maps: Vec<Vec<Range>> = Vec::new();
    for position in map_positions {
        all_maps.push(get_map_ranges(lines, position.0, position.1));
    }
    all_maps
}

fn get_map_ranges(lines: &Vec<String>, map_start: usize, map_end: usize) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();
    for i in map_start..(map_end + 1) {
        let numbers: Vec<i64> = lines[i].split_whitespace()
                                        .map(|s| s.parse().expect("Parsing error"))
                                        .collect();
        ranges.push(Range{dst_start: numbers[0], src_start: numbers[1], length: numbers[2]});
    }
    ranges
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
