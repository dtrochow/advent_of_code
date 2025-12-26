use aoc_lib::{read_lines, Point};
use std::collections::HashMap;

type Map = Vec<Vec<char>>;
type Beam = Point;

fn get_map(lines: Vec<String>) -> Map {
    lines.iter().map(|l| l.chars().collect()).collect()
}

fn get_start(map: &Map) -> Point {
    let pos = map.first().unwrap().iter().position(|x| *x == 'S').unwrap() as i64;
    Point { x: pos, y: 0 }
}

fn run_beams(cache: &mut HashMap<Point, u64>, map: &Map, pos: Beam, splits: u64) -> u64 {
    if pos.y as usize == map.len() - 1 {
        return splits;
    }
    let new_pos = Point {
        x: pos.x,
        y: pos.y + 1,
    };
    if cache.contains_key(&new_pos) {
        return cache[&new_pos];
    }
    if map[new_pos.y as usize][new_pos.x as usize] == '^' {
        let left = Point {
            x: new_pos.x - 1,
            y: new_pos.y,
        };
        let right = Point {
            x: new_pos.x + 1,
            y: new_pos.y,
        };
        let value = run_beams(cache, map, left, splits) + run_beams(cache, map, right, splits);
        cache.insert(pos, value);
        value
    } else {
        let value = run_beams(cache, map, new_pos, splits);
        cache.insert(pos, value);
        value
    }
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let map = get_map(lines);

    let start = get_start(&map);
    println!("Start: {:?}", start);

    let mut cache: HashMap<Point, u64> = HashMap::new();
    let number_of_splits = run_beams(&mut cache, &map, start, 1);
    println!("Number of beams: {}", number_of_splits);
}
