use aoc_lib::{read_lines, Point};

type Map = Vec<Vec<char>>;
type Beam = Point;

fn get_map(lines: Vec<String>) -> Map {
    lines.iter().map(|l| l.chars().collect()).collect()
}

fn get_start(map: &Map) -> Point {
    let pos = map.first().unwrap().iter().position(|x| *x == 'S').unwrap() as i64;
    Point { x: pos, y: 0 }
}

fn move_beams(map: &Map, beams: &mut Vec<Beam>) -> u64 {
    let mut number_of_splits = 0;
    let mut new_beams: Vec<Beam> = Vec::new();

    for beam in beams.iter_mut() {
        if map[(beam.y + 1) as usize][beam.x as usize] != '^' {
            beam.y += 1;
        } else {
            new_beams.push(Beam {
                x: beam.x + 1,
                y: beam.y + 1,
            });
            beam.y += 1;
            beam.x -= 1;
            number_of_splits += 1;
        }
    }

    beams.extend(new_beams);
    beams.sort_unstable();
    beams.dedup();

    number_of_splits
}

fn are_all_beams_out(beams: &[Beam], height: usize) -> bool {
    beams.iter().all(|b| b.y >= (height - 1) as i64)
}

fn run_beams(map: Map, start: Point) -> u64 {
    let mut number_of_splits = 0;
    let mut beams: Vec<Beam> = vec![start];

    loop {
        number_of_splits += move_beams(&map, &mut beams);
        if are_all_beams_out(&beams, map.len()) {
            break;
        }
    }
    number_of_splits
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let map = get_map(lines);

    let start = get_start(&map);
    println!("Start: {:?}", start);

    let number_of_splits = run_beams(map, start);
    println!("Number of splits: {}", number_of_splits);
}
