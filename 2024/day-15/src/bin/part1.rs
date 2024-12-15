use aoc_lib::{find_index, is_in_2d_map, read_lines, Direction, Point};

enum CellType {
    Wall,
    Box,
    Empty,
}

impl CellType {
    fn value(&self) -> char {
        match self {
            CellType::Box => 'O',
            CellType::Wall => '#',
            CellType::Empty => '.',
        }
    }
}

fn char_to_cell(c: char) -> CellType {
    match c {
        'O' => CellType::Box,
        '#' => CellType::Wall,
        '.' => CellType::Empty,
        _ => panic!("<{}> char cannot be converted to CellType", c),
    }
}

type Warehouse = Vec<Vec<char>>;
type MovesSequence = Vec<Direction>;

fn get_warehouse(lines: &[String]) -> Warehouse {
    let end_index = find_index(lines, &"".to_string()).unwrap();

    lines[..end_index]
        .iter()
        .map(|l| l.chars().collect())
        .collect()
}

fn find_robot_pos(warehouse: &Warehouse) -> Option<Point> {
    const ROBOT: char = '@';
    for (y, row) in warehouse.iter().enumerate() {
        if let Some(x) = find_index(row, &ROBOT) {
            return Some(Point {
                x: x as i64,
                y: y as i64,
            });
        }
    }
    None
}

fn get_moves_sequence(lines: &[String]) -> MovesSequence {
    let start_index = find_index(lines, &"".to_string()).unwrap() + 1;

    lines[start_index..]
        .iter()
        .flat_map(|l| {
            l.chars().filter_map(|c| match c {
                '^' => Some(Direction::Up),
                '>' => Some(Direction::Right),
                '<' => Some(Direction::Left),
                'v' => Some(Direction::Down),
                _ => None,
            })
        })
        .collect()
}

fn can_move_box(warehouse: &Warehouse, box_pos: &Point, dir: &Direction) -> bool {
    let mut next_pos = *box_pos + dir.point();
    while is_in_2d_map(warehouse, &next_pos) {
        let cell = warehouse[next_pos.y as usize][next_pos.x as usize];
        if cell == CellType::Wall.value() {
            return false;
        } else if cell == CellType::Empty.value() {
            return true;
        }
        next_pos = next_pos + dir.point();
    }
    false
}

fn swap_cells(warehouse: &mut Warehouse, pos1: &Point, pos2: &Point) {
    let tmp = warehouse[pos1.y as usize][pos1.x as usize];
    warehouse[pos1.y as usize][pos1.x as usize] = warehouse[pos2.y as usize][pos2.x as usize];
    warehouse[pos2.y as usize][pos2.x as usize] = tmp;
}

fn move_box(warehouse: &mut Warehouse, box_pos: &Point, dir: &Direction) {
    /* find the empty space */
    let mut next_pos = *box_pos + dir.point();
    let mut cell = warehouse[next_pos.y as usize][next_pos.x as usize];
    while cell != CellType::Empty.value() {
        next_pos = next_pos + dir.point();
        cell = warehouse[next_pos.y as usize][next_pos.x as usize];
    }

    let mut empty_pos = next_pos;
    /* move all boxes in the empty space direction */
    while next_pos != (*box_pos - dir.point()) {
        next_pos = empty_pos - dir.point();
        swap_cells(warehouse, &next_pos, &empty_pos);
        empty_pos = next_pos;
    }
}

fn move_robot(warehouse: &mut Warehouse, robot_pos: &mut Point, moves_seq: MovesSequence) {
    for m in moves_seq {
        let next_pos = *robot_pos + m.point();
        let next_pos_cell = warehouse[next_pos.y as usize][next_pos.x as usize];
        match char_to_cell(next_pos_cell) {
            CellType::Wall => continue,
            CellType::Box => {
                if can_move_box(warehouse, &next_pos, &m) {
                    move_box(warehouse, &next_pos, &m);
                    *robot_pos = next_pos;
                }
            }
            CellType::Empty => {
                *robot_pos = next_pos;
            }
        }
    }
}

fn calculate_gps_coordinates(warehouse: &Warehouse) -> u64 {
    warehouse
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter().enumerate().filter_map(move |(x, c)| {
                if *c == CellType::Box.value() {
                    Some(x as u64 + (100 * y as u64))
                } else {
                    None
                }
            })
        })
        .sum()
}

fn print_warehouse(warehouse: &Warehouse, robot_pos: &Point) {
    let mut warehouse_cpy = warehouse.clone();
    warehouse_cpy[robot_pos.y as usize][robot_pos.x as usize] = '@';
    for row in warehouse_cpy {
        for x in row {
            print!("{}", x);
        }
        println!();
    }
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let mut warehouse = get_warehouse(&lines);
    let mut robot_pos = find_robot_pos(&warehouse).unwrap();
    warehouse[robot_pos.y as usize][robot_pos.x as usize] = CellType::Empty.value();
    let moves_seq = get_moves_sequence(&lines);

    move_robot(&mut warehouse, &mut robot_pos, moves_seq);
    print_warehouse(&warehouse, &robot_pos);

    let gps_coordinates = calculate_gps_coordinates(&warehouse);
    println!("GPS Coordinates: {}", gps_coordinates);
}
