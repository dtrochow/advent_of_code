use std::fs::read_to_string;
use std::time::Instant;

type Grid = Vec<Vec<char>>;

#[repr(u8)]
enum TileType {
    Empty = b'.',
    RightHeadedMirror = b'/',
    LeftHeadedMirror = b'\\',
    HorizontalLine = b'-',
    VerticalLine = b'|',
}

struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Beam {
    direction: Direction,
    energized_tiles: Vec<Position>,
}

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut grid: Grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }

    let mut beams: Vec<Beam> = vec![Beam {
        direction: Direction::Right,
        energized_tiles: vec![Position { x: 0, y: 0 }],
    }];
    spread_the_beams(&grid, &mut beams);

    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

fn spread_the_beams(grid: &Grid, beams: &mut [Beam]) {
    while beams.iter().any(|b| !is_beam_end(grid, b)) {
        for beam in beams.iter_mut() {
            if !is_beam_end(grid, beam) {}
        }
    }
}

fn expand_beam(beam: &mut Beam) {}

fn is_beam_end(grid: &Grid, beam: &Beam) -> bool {
    let position: &Position = beam.energized_tiles.last().unwrap();
    let direction: Direction = beam.direction.clone();

    let grid_x_size = grid.first().unwrap().len();
    let grid_y_size = grid.len();

    ((position.x == 0) && (direction == Direction::Left))
        || ((position.x == (grid_x_size - 1)) && (direction == Direction::Right))
        || ((position.y == 0) && (direction == Direction::Up))
        || ((position.x == (grid_y_size - 1)) && (direction == Direction::Down))
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
