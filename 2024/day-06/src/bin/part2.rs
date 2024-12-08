use aoc_lib::read_lines;

type LabMap = Vec<Vec<char>>;

const OBSTACLE: char = '#';

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

/*
     y0 y1 y2 y3 y4
  x0  .  .  .  . .
  x1  .  .  .  . .
  x2  .  .  .  . .
  x3  .  .  .  . .
  x4  .  .  .  . .
*/
fn get_lab_map(lines: Vec<String>) -> LabMap {
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_guard_position(lab_map: &LabMap) -> Option<Position> {
    for (x, row) in lab_map.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if *value == '^' {
                return Some(Position { x, y });
            }
        }
    }
    None
}

fn is_leaving(lab_map: &LabMap, current_pos: &Position, direction: &Direction) -> bool {
    match direction {
        Direction::Up => current_pos.x == 0,
        Direction::Down => current_pos.x == lab_map.len() - 1,
        Direction::Left => current_pos.y == 0,
        Direction::Right => current_pos.y == lab_map.first().unwrap().len() - 1,
    }
}

fn rotate_until_free_path(
    lab_map: &mut LabMap,
    current_pos: &mut Position,
    direction: &mut Direction,
) {
    let mut next_pos;
    loop {
        match direction {
            Direction::Up => {
                next_pos = Position {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                };
                if lab_map[next_pos.x][next_pos.y] == OBSTACLE {
                    *direction = Direction::Right;
                }
            }
            Direction::Down => {
                next_pos = Position {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                };
                if lab_map[next_pos.x][next_pos.y] == OBSTACLE {
                    *direction = Direction::Left;
                }
            }
            Direction::Left => {
                next_pos = Position {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                };
                if lab_map[next_pos.x][next_pos.y] == OBSTACLE {
                    *direction = Direction::Up;
                }
            }
            Direction::Right => {
                next_pos = Position {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                };
                if lab_map[next_pos.x][next_pos.y] == OBSTACLE {
                    *direction = Direction::Down;
                }
            }
        };
        if lab_map[next_pos.x][next_pos.y] != '#' {
            break;
        }
    }
}

fn move_guard(lab_map: &mut LabMap, current_pos: &mut Position, direction: &mut Direction) {
    rotate_until_free_path(lab_map, current_pos, direction);
    match direction {
        Direction::Up => {
            *current_pos = Position {
                x: current_pos.x - 1,
                y: current_pos.y,
            };
        }
        Direction::Down => {
            *current_pos = Position {
                x: current_pos.x + 1,
                y: current_pos.y,
            };
        }
        Direction::Left => {
            *current_pos = Position {
                x: current_pos.x,
                y: current_pos.y - 1,
            };
        }
        Direction::Right => {
            *current_pos = Position {
                x: current_pos.x,
                y: current_pos.y + 1,
            };
        }
    };
    lab_map[current_pos.x][current_pos.y] = 'X';
}

fn is_obstruct_creates_loop(
    lab_map: &LabMap,
    current_pos: &Position,
    direction: &Direction,
    guard_start_pos: &Position,
) -> bool {
    let mut temp_lab_map = lab_map.clone();
    let mut temp_current_pos = current_pos.clone();
    let mut temp_direction = direction.clone();

    let obstruct_position;
    match direction {
        Direction::Up => {
            obstruct_position = Position {
                x: current_pos.x - 1,
                y: current_pos.y,
            };
        }
        Direction::Down => {
            obstruct_position = Position {
                x: current_pos.x + 1,
                y: current_pos.y,
            };
        }
        Direction::Left => {
            obstruct_position = Position {
                x: current_pos.x,
                y: current_pos.y - 1,
            };
        }
        Direction::Right => {
            obstruct_position = Position {
                x: current_pos.x,
                y: current_pos.y + 1,
            };
        }
    }

    if obstruct_position == *guard_start_pos {
        return false;
    }

    /* Put new obstruction */
    temp_lab_map[obstruct_position.x][obstruct_position.y] = '#';

    let mut remembered_pos: Vec<(Position, Direction, Position, Direction)> = Vec::new();
    while !is_leaving(&temp_lab_map, &temp_current_pos, &temp_direction) {
        let previous_pos = temp_current_pos.clone();
        let previous_dir = temp_direction.clone();
        move_guard(
            &mut temp_lab_map,
            &mut temp_current_pos,
            &mut temp_direction,
        );
        if remembered_pos.contains(&(
            previous_pos.clone(),
            previous_dir.clone(),
            temp_current_pos.clone(),
            temp_direction.clone(),
        )) {
            // println!("Obstruction: {:?} Current: {:?} {:?}", obstruct_position, temp_current_pos, temp_direction);
            // for pos in remembered_pos {
            //     println!("{:?}", pos);
            // }
            return true;
        }
        remembered_pos.push((
            previous_pos.clone(),
            previous_dir.clone(),
            temp_current_pos.clone(),
            temp_direction.clone(),
        ));
    }
    false
}

fn start_guard(lab_map: &mut LabMap, guard_pos: Position) -> u64 {
    let guard_start_pos = guard_pos.clone();
    let mut current_pos: Position = guard_pos;
    let mut direction: Direction = Direction::Up;

    /* Mark position as visited - vertical direction */
    lab_map[current_pos.x][current_pos.y] = 'X';

    let mut loop_creating_obstructs_count = 0;
    while !is_leaving(lab_map, &current_pos, &direction) {
        if is_obstruct_creates_loop(lab_map, &current_pos, &direction, &guard_start_pos) {
            loop_creating_obstructs_count += 1;
        }
        move_guard(lab_map, &mut current_pos, &mut direction);
    }
    loop_creating_obstructs_count
}

fn main() {
    let lines = read_lines("./src/bin/input2.txt");

    let mut lab_map: LabMap = get_lab_map(lines);
    let guard_pos = find_guard_position(&lab_map).unwrap();
    let loop_creating_obstructs_count = start_guard(&mut lab_map, guard_pos);

    println!(
        "Loop creating obstructs count: {}",
        loop_creating_obstructs_count
    );
}
