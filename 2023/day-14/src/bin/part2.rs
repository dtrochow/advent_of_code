use std::fs::read_to_string;
use std::time::Instant;

type Grid = Vec<Vec<char>>;

const EMPTY: char = '.';
const ROUND_ROCK: char = 'O';

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut grid: Grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }

    let mut seen: Vec<Grid> = vec![grid.clone()];
    loop {
        perform_cycle(&mut grid);
        if let Some(idx) = seen.iter().position(|x| x == &grid) {
            let cycle_len = seen.len() - idx;
            let last_grid_idx =  idx + (1000_000_000 - idx) % cycle_len;
            let load = measure_load(&seen[last_grid_idx]);
            println!("Load: {}", load);
            break;
        }
        seen.push(grid.clone());
    }

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn perform_cycle(grid: &mut Grid) -> () {
    for _ in 0..4 {
        tilt_to_the_north(grid);
        *grid = rotate_clockwise(grid);
    }
}

fn rotate_clockwise(grid: &Grid) -> Grid {
    let size = grid.len();
    let mut rotated_grid: Grid = vec![vec![EMPTY; size]; size];
    for i in 0..size {
        for j in 0..size {
            rotated_grid[i][j] = grid[size - 1 -j][i];
        }
    }
    rotated_grid
}

fn measure_load(grid: &Grid) -> u32 {
    let mut load: usize = 0;
    for (index, row) in grid.iter().enumerate() {
        for ch in row {
            if *ch == ROUND_ROCK {
                load += grid.len() - index;
            }
        }
    }
    load as u32
}

fn tilt_to_the_north(grid: &mut Grid) -> () {
    let horizontal_size = grid.first().unwrap().len();
    for column in 0..horizontal_size {
        move_round_rocks_in_column(grid, column);
    }
}

fn move_round_rocks_in_column(grid: &mut Grid, column: usize) -> () {
    let column_size = grid.len();

    for column_index in 1..column_size {
        if grid[column_index][column] == ROUND_ROCK {
            let first_empty = find_first_empty(grid, column, column_index);
            if first_empty.is_some() {
                move_rock(grid, column, column_index, first_empty.unwrap());
            }
        }
    }
}

fn move_rock(grid: &mut Grid, column: usize, column_index_1: usize, column_index_2: usize) -> () {
    let tmp = grid[column_index_1][column];
    grid[column_index_1][column] = grid[column_index_2][column];
    grid[column_index_2][column] = tmp;
}

fn find_first_empty(grid: &Grid, column: usize, current_column_index: usize) -> Option<usize> {
    for index in (0..current_column_index).rev() {
        if grid[index][column] != EMPTY {
            return Some(index + 1);
        } else if index == 0 && grid[index][column] == EMPTY {
            return Some(0);
        }
    }
    None
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
