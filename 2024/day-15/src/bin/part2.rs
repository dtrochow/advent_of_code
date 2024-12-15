use aoc_lib::{find_index, is_in_2d_map2, read_lines, Direction, Point};

enum CellType {
    Wall,
    BoxL,
    BoxR,
    Empty,
}

impl CellType {
    fn value(&self) -> char {
        match self {
            CellType::BoxL => '[',
            CellType::BoxR => ']',
            CellType::Wall => '#',
            CellType::Empty => '.',
        }
    }
}

fn char_to_cell(c: char) -> CellType {
    match c {
        '[' => CellType::BoxL,
        ']' => CellType::BoxR,
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

fn can_move_vertically(
    warehouse: &Warehouse,
    next_pos: &Point,
    dir: &Direction,
    boxes: &mut Vec<(Point, CellType)>,
) -> bool {
    let cell = warehouse[next_pos.y as usize][next_pos.x as usize];
    match char_to_cell(cell) {
        CellType::Empty => true,
        CellType::Wall => false,
        CellType::BoxL => {
            boxes.push((*next_pos, CellType::BoxL));
            boxes.push((*next_pos + Direction::Right.point(), CellType::BoxR));
            can_move_vertically(warehouse, &(*next_pos + dir.point()), dir, boxes)
                && can_move_vertically(
                    warehouse,
                    &(*next_pos + dir.point() + Direction::Right.point()),
                    dir,
                    boxes,
                )
        }
        CellType::BoxR => {
            boxes.push((*next_pos, CellType::BoxR));
            boxes.push((*next_pos + Direction::Left.point(), CellType::BoxL));
            can_move_vertically(warehouse, &(*next_pos + dir.point()), dir, boxes)
                && can_move_vertically(
                    warehouse,
                    &(*next_pos + dir.point() + Direction::Left.point()),
                    dir,
                    boxes,
                )
        }
    }
}

fn can_move_box(
    warehouse: &Warehouse,
    box_pos: &Point,
    dir: &Direction,
) -> Option<Vec<(Point, CellType)>> {
    let mut next_pos = *box_pos;
    let mut boxes: Vec<(Point, CellType)> = Vec::new();
    if *dir == Direction::Left || *dir == Direction::Right {
        while is_in_2d_map2(warehouse, &next_pos) {
            let cell = warehouse[next_pos.y as usize][next_pos.x as usize];
            match char_to_cell(cell) {
                CellType::Wall => {
                    return None;
                }
                CellType::Empty => {
                    return Some(boxes);
                }
                CellType::BoxL | CellType::BoxR => {
                    boxes.push((next_pos, char_to_cell(cell)));
                }
            }
            next_pos = next_pos + dir.point();
        }
    } else if can_move_vertically(warehouse, &next_pos, dir, &mut boxes) {
        return Some(boxes);
    }
    None
}

fn move_box(warehouse: &mut Warehouse, dir: &Direction, boxes_to_move: Vec<(Point, CellType)>) {
    for b in &boxes_to_move {
        let box_pos = b.0;
        warehouse[box_pos.y as usize][box_pos.x as usize] = CellType::Empty.value();
    }
    for b in boxes_to_move {
        let next_box_pos = b.0 + dir.point();
        warehouse[next_box_pos.y as usize][next_box_pos.x as usize] = b.1.value();
    }
}

fn move_robot(warehouse: &mut Warehouse, robot_pos: &mut Point, moves_seq: MovesSequence) {
    for m in moves_seq {
        let next_pos = *robot_pos + m.point();
        let next_pos_cell = warehouse[next_pos.y as usize][next_pos.x as usize];
        match char_to_cell(next_pos_cell) {
            CellType::Wall => continue,
            CellType::BoxL | CellType::BoxR => {
                if let Some(boxes_to_move) = can_move_box(warehouse, &next_pos, &m) {
                    move_box(warehouse, &m, boxes_to_move);
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
                if *c == CellType::BoxL.value() {
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

fn build_extended_warehouse(warehouse: &Warehouse) -> Warehouse {
    let mut ext_warehouse: Warehouse = Vec::new();
    ext_warehouse.push(
        "#".repeat(warehouse.first().unwrap().len() * 2)
            .chars()
            .collect(),
    );
    for row in warehouse[1..warehouse.len() - 1].iter() {
        let mut new_row: Vec<char> = Vec::new();
        for ch in row {
            match ch {
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => panic!("unexpected character: <{}>", ch),
            }
        }
        ext_warehouse.push(new_row);
    }
    ext_warehouse.push(
        "#".repeat(warehouse.first().unwrap().len() * 2)
            .chars()
            .collect(),
    );

    ext_warehouse
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let mut warehouse = get_warehouse(&lines);
    warehouse = build_extended_warehouse(&warehouse);
    let mut robot_pos = find_robot_pos(&warehouse).unwrap();
    warehouse[robot_pos.y as usize][robot_pos.x as usize] = CellType::Empty.value();
    let moves_seq = get_moves_sequence(&lines);

    move_robot(&mut warehouse, &mut robot_pos, moves_seq);
    print_warehouse(&warehouse, &robot_pos);

    let gps_coordinates = calculate_gps_coordinates(&warehouse);
    println!("GPS Coordinates: {}", gps_coordinates);
}
