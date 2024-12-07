use aoc_lib::read_lines;

type LabMap = Vec<Vec<char>>;

const OBSTACLE: char = '#';

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
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

fn move_guard(lab_map: &mut LabMap, current_pos: &mut Position, direction: &mut Direction) {
    match direction {
        Direction::Up => {
            if lab_map[current_pos.x - 1][current_pos.y] == OBSTACLE {
                *current_pos = Position {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                };
                *direction = Direction::Right;
            } else {
                *current_pos = Position {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                };
            }
        }
        Direction::Down => {
            if lab_map[current_pos.x + 1][current_pos.y] == OBSTACLE {
                *current_pos = Position {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                };
                *direction = Direction::Left;
            } else {
                *current_pos = Position {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                };
            }
        }
        Direction::Left => {
            if lab_map[current_pos.x][current_pos.y - 1] == OBSTACLE {
                *current_pos = Position {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                };
                *direction = Direction::Up;
            } else {
                *current_pos = Position {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                };
            }
        }
        Direction::Right => {
            if lab_map[current_pos.x][current_pos.y + 1] == OBSTACLE {
                *current_pos = Position {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                };
                *direction = Direction::Down;
            } else {
                *current_pos = Position {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                };
            }
        }
    };
    /* Mark position as visited */
    lab_map[current_pos.x][current_pos.y] = 'X';
}

fn start_guard(lab_map: &mut LabMap, guard_pos: Position) {
    let mut current_pos: Position = guard_pos;
    let mut direction: Direction = Direction::Up;

    /* Mark position as visited */
    lab_map[current_pos.x][current_pos.y] = 'X';

    while !is_leaving(lab_map, &current_pos, &direction) {
        move_guard(lab_map, &mut current_pos, &mut direction);
    }
}

fn count_distinct_positions(lab_map: &LabMap) -> u32 {
    lab_map
        .iter()
        .flatten()
        .filter(|&&value| value == 'X')
        .count() as u32
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let mut lab_map: LabMap = get_lab_map(lines);
    let guard_pos = find_guard_position(&lab_map).unwrap();
    start_guard(&mut lab_map, guard_pos);

    println!("Distinct positions: {}", count_distinct_positions(&lab_map));
}
