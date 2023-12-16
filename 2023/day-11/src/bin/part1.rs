use std::fs::read_to_string;
use std::time::Instant;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let now = Instant::now();

    let mut galaxy: Vec<String> = read_lines("./src/bin/input1.txt");
    expand_galaxy(&mut galaxy);

    let galaxies_positions: Vec<Position> = find_galaxies_positions(galaxy);

    let mut distances_sum: u32 = 0;
    for (index, pos) in galaxies_positions.iter().enumerate() {
        for i in (index + 1)..galaxies_positions.len() {
            distances_sum += calculate_distance(pos, galaxies_positions.get(i).unwrap());
        }
    }
    println!("Distances sum: {}", distances_sum);

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}



fn calculate_distance(pos1: &Position, pos2: &Position) -> u32 {
    let x_distance: i32 = pos2.x - pos1.x;
    let y_distance: i32 = pos2.y - pos1.y;
    (y_distance.abs() + x_distance.abs()) as u32
}

fn find_galaxies_positions(lines: Vec<String>) -> Vec<Position> {
    let mut galaxies_positions: Vec<Position> = Vec::new();

    for (y, row) in lines.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                galaxies_positions.push(Position{x: x as i32, y: y as i32});
            }
        }
    }
    galaxies_positions
}

fn expand_galaxy(galaxy: &mut Vec<String>) -> () {
    let rows_to_expand = find_rows_to_expand(galaxy);
    let columns_to_expand = find_columns_to_expand(galaxy);

    let empty_row: String = vec!['.'; galaxy.first().unwrap().len()].iter().collect();
    for row in rows_to_expand {
        galaxy.insert(row as usize, empty_row.clone());
    }

    for column in columns_to_expand {
        for row in galaxy.iter_mut() {
            row.insert(column as usize, '.');
        }
    }
}

fn find_rows_to_expand(lines: &Vec<String>) -> Vec<u32> {
    let mut rows_to_expand: Vec<u32> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if !line.contains('#') {
            rows_to_expand.push(index.try_into().unwrap());
        }
    }
    rows_to_expand.sort();
    rows_to_expand.reverse();
    rows_to_expand
}

fn find_columns_to_expand(lines: &Vec<String>) -> Vec<u32> {
    let mut columns_to_expand: Vec<u32> = Vec::new();
    for index in 0..lines.first().unwrap().len() {
        if is_column_to_expand(lines, index) {
            columns_to_expand.push(index.try_into().unwrap());
        }
    }
    columns_to_expand.sort();
    columns_to_expand.reverse();
    columns_to_expand
}

fn is_column_to_expand(lines: &Vec<String>, column: usize) -> bool {
    for line in lines {
        if line.chars().nth(column).unwrap() == '#' {
            return false;
        }
    }
    true
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
