use std::fs::read_to_string;
use std::time::Instant;
use std::fmt;

type PipesMap = Vec<Vec<char>>;

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

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let mut pipes_map: PipesMap = Vec::new();

    for line in lines {
        pipes_map.push(line.chars().collect());
    }

    let start_pos: Position = find_start_position(&pipes_map).unwrap();
    println!("Start {:?}", start_pos);

    let mut previous_pos: Position = start_pos.clone();
    let mut current_pos: Position = find_first_position_after_start(&start_pos, &pipes_map).unwrap();
    let mut number_of_steps: u64 = 1;

    loop {
        let remember_previous_pos = current_pos.clone();

        let cur_pipe_type: char = get_pipe_type(&current_pos, &pipes_map);
        current_pos = get_next_pos(cur_pipe_type, &current_pos, &previous_pos).unwrap();
        previous_pos = remember_previous_pos;

        // println!("Previous {:?} Pipe: {} | Current {:?} Pipe: {}\n", 
        // previous_pos, get_pipe_type(&previous_pos, &pipes_map),
        // current_pos, get_pipe_type(&current_pos, &pipes_map));

        number_of_steps += 1;
        if (current_pos == start_pos) || (get_pipe_type(&current_pos, &pipes_map) == '.') { break; }
    }

    println!("Max number of steps to animal: {}", number_of_steps/2);

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn get_next_pos(cur_pipe_type: char, current_pos: &Position, previous_pos: &Position) -> Option<Position> {
    let vertical_diff: i32  = current_pos.y - previous_pos.y;
    let horizontal_diff: i32 = current_pos.x - previous_pos.x;

    // Worth to remember that Y indexes are GROWING DOWN to the map
    match cur_pipe_type {
        '|' => {
            if vertical_diff < 0 {
                return Some(Position{x: current_pos.x, y: current_pos.y-1});
            } else {
                return Some(Position{x: current_pos.x, y: current_pos.y+1});
            }
        },
        '-' => {
            if horizontal_diff > 0 {
                return Some(Position{x: current_pos.x+1, y: current_pos.y});
            } else {
                return Some(Position{x: current_pos.x-1, y: current_pos.y});
            }        
        },
        'L' => {
            if vertical_diff > 0 {
                return Some(Position{x: current_pos.x+1, y: current_pos.y});
            } else if horizontal_diff < 0 {
                return Some(Position{x: current_pos.x, y: current_pos.y-1});
            } else {
                None
            }
        },
        'J' => {
            if vertical_diff > 0 {
                return Some(Position{x: current_pos.x-1, y: current_pos.y});
            } else if horizontal_diff > 0 {
                return Some(Position{x: current_pos.x, y: current_pos.y-1});
            } else {
                None
            }
        },
        '7' => {
            if vertical_diff < 0 {
                return Some(Position{x: current_pos.x-1, y: current_pos.y});
            } else if horizontal_diff > 0 {
                return Some(Position{x: current_pos.x, y: current_pos.y+1});
            } else {
                None
            }
        },
        'F' => {
            if vertical_diff < 0 {
                return Some(Position{x: current_pos.x+1, y: current_pos.y});
            } else if horizontal_diff < 0 {
                return Some(Position{x: current_pos.x, y: current_pos.y+1});
            } else {
                None
            }
        },
        _ => { return None; }
    }
}

fn get_pipe_type(position: &Position, pipes_map: &PipesMap) -> char {
    pipes_map[position.y as usize][position.x as usize]
}

fn find_first_position_after_start(start_pos: &Position, pipes_map: &PipesMap) -> Option<Position> {
    let up_pos = Position{x: start_pos.x, y: start_pos.y-1};
    if vec!['|', 'F', '7'].contains(&get_pipe_type(&up_pos, &pipes_map)) {
       return Some(up_pos);
    }

    let down_pos = Position{x: start_pos.x, y: start_pos.y+1};
    if vec!['|', 'L', 'J'].contains(&get_pipe_type(&down_pos, &pipes_map)) {
        return Some(down_pos);
    }

    let left_pos = Position{x: start_pos.x-1, y: start_pos.y};
    if vec!['-', 'F', 'L'].contains(&get_pipe_type(&left_pos, &pipes_map)) {
        return Some(left_pos);
    }
    
    let right_pos = Position{x: start_pos.x+1, y: start_pos.y};
    if vec!['-', 'J', '7'].contains(&get_pipe_type(&right_pos, &pipes_map)) {
        return Some(right_pos);
    }

    None
}

fn find_start_position(pipes_map: &PipesMap) -> Option<Position> {
    for x in 0..pipes_map.first().unwrap().len() {
        for y in 0..pipes_map.len() {
            if pipes_map[y][x] == 'S' {
                return Some(Position{x: x as i32, y: y as i32});
            }
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
