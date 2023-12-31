use std::fmt;
use std::fs::read_to_string;
use std::time::Instant;

type PipesMap = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
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

    let mut pipe_loop_pos: Vec<Position> = Vec::new();
    pipe_loop_pos.push(find_start_position(&pipes_map).unwrap());
    println!("Start {:?}", pipe_loop_pos.first().unwrap());

    pipe_loop_pos
        .push(find_first_position_after_start(pipe_loop_pos.first().unwrap(), &pipes_map).unwrap());
    let mut number_of_steps: u64 = 1;

    loop {
        let current_pos = pipe_loop_pos.last().unwrap();
        let previous_pos = &pipe_loop_pos[pipe_loop_pos.len() - 2];
        let cur_pipe_type: char = get_pipe_type(current_pos, &pipes_map);
        pipe_loop_pos.push(get_next_pos(cur_pipe_type, current_pos, previous_pos).unwrap());

        // println!("Previous {:?} Pipe: {} | Current {:?} Pipe: {}\n",
        // previous_pos, get_pipe_type(&previous_pos, &pipes_map),
        // current_pos, get_pipe_type(&current_pos, &pipes_map));

        number_of_steps += 1;

        let start_pos = pipe_loop_pos.first().unwrap();
        if pipe_loop_pos.last().unwrap() == start_pos {
            break;
        }
    }

    let enclosed_tiles_count: u32 = count_enclosed_tiles(&pipe_loop_pos, &pipes_map);

    println!("Pipe Loop Positions: {}", pipe_loop_pos.len());
    println!("Max number of steps to animal: {}", number_of_steps / 2);
    println!("Enclosed tiles count: {}", enclosed_tiles_count);

    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

fn count_enclosed_tiles(pipe_loop_pos: &[Position], pipes_map: &PipesMap) -> u32 {
    let mut potential_enclosed_tiles: Vec<Position> = Vec::new();

    for y in 0..pipes_map.len() {
        let mut is_inside: bool = false;
        for x in 0..pipes_map.first().unwrap().len() {
            let position: Position = Position {
                x: x as i32,
                y: y as i32,
            };

            if pipe_loop_pos.contains(&position) && !is_inside {
                is_inside = true;
            } else if pipe_loop_pos.contains(&position) && is_inside {
                is_inside = false;
            }

            if !pipe_loop_pos.contains(&position) && is_inside {
                potential_enclosed_tiles.push(position);
            }
        }
    }

    0
}

fn get_next_pos(
    cur_pipe_type: char,
    current_pos: &Position,
    previous_pos: &Position,
) -> Option<Position> {
    let vertical_diff: i32 = current_pos.y - previous_pos.y;
    let horizontal_diff: i32 = current_pos.x - previous_pos.x;

    // Worth to remember that Y indexes are GROWING DOWN to the map
    match cur_pipe_type {
        '|' => {
            if vertical_diff < 0 {
                Some(Position {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                })
            } else {
                Some(Position {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                })
            }
        }
        '-' => {
            if horizontal_diff > 0 {
                Some(Position {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                })
            } else {
                Some(Position {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                })
            }
        }
        'L' => {
            if vertical_diff > 0 {
                Some(Position {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                })
            } else if horizontal_diff < 0 {
                Some(Position {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                })
            } else {
                None
            }
        }
        'J' => {
            if vertical_diff > 0 {
                Some(Position {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                })
            } else if horizontal_diff > 0 {
                Some(Position {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                })
            } else {
                None
            }
        }
        '7' => {
            if vertical_diff < 0 {
                Some(Position {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                })
            } else if horizontal_diff > 0 {
                Some(Position {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                })
            } else {
                None
            }
        }
        'F' => {
            if vertical_diff < 0 {
                Some(Position {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                })
            } else if horizontal_diff < 0 {
                Some(Position {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_pipe_type(position: &Position, pipes_map: &PipesMap) -> char {
    pipes_map[position.y as usize][position.x as usize]
}

fn find_first_position_after_start(start_pos: &Position, pipes_map: &PipesMap) -> Option<Position> {
    let up_pos = Position {
        x: start_pos.x,
        y: start_pos.y - 1,
    };
    if vec!['|', 'F', '7'].contains(&get_pipe_type(&up_pos, pipes_map)) {
        return Some(up_pos);
    }

    let down_pos = Position {
        x: start_pos.x,
        y: start_pos.y + 1,
    };
    if vec!['|', 'L', 'J'].contains(&get_pipe_type(&down_pos, pipes_map)) {
        return Some(down_pos);
    }

    let left_pos = Position {
        x: start_pos.x - 1,
        y: start_pos.y,
    };
    if vec!['-', 'F', 'L'].contains(&get_pipe_type(&left_pos, pipes_map)) {
        return Some(left_pos);
    }

    let right_pos = Position {
        x: start_pos.x + 1,
        y: start_pos.y,
    };
    if vec!['-', 'J', '7'].contains(&get_pipe_type(&right_pos, pipes_map)) {
        return Some(right_pos);
    }

    None
}

fn find_start_position(pipes_map: &PipesMap) -> Option<Position> {
    for (y, row) in pipes_map.iter().enumerate() {
        for (x, pipe_type) in row.iter().enumerate() {
            if *pipe_type == 'S' {
                return Some(Position {
                    x: x as i32,
                    y: y as i32,
                });
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
