use aoc_lib::read_lines;

type ForkliftMatrix = Vec<Vec<char>>;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn is_accessible_by_forklift(matrix: &ForkliftMatrix, position: &Position) -> bool {
    let x: i32 = position.x;
    let y: i32 = position.y;

    /*
    X X X
    X o X
    X X X
    */
    let neighbor_positions: Vec<Position> = vec![
        Position { x: x - 1, y: y - 1 },
        Position { x, y: y - 1 },
        Position { x: x + 1, y: y - 1 },
        Position { x: x - 1, y },
        Position { x: x + 1, y },
        Position { x: x - 1, y: y + 1 },
        Position { x, y: y + 1 },
        Position { x: x + 1, y: y + 1 },
    ];

    let mut neighbor_elements: Vec<char> = Vec::new();
    for pos in neighbor_positions {
        if pos.x >= 0
            && pos.y >= 0
            && pos.y < matrix.len() as i32
            && pos.x < matrix.first().unwrap().len() as i32
        {
            neighbor_elements.push(matrix[pos.y as usize][pos.x as usize]);
        }
    }

    let count = neighbor_elements.iter().filter(|&n| *n == '@').count();

    count < 4
}

fn remove_rolls(matrix: &mut ForkliftMatrix, remove_positions: Vec<Position>) {
    for pos in remove_positions {
        matrix[pos.y as usize][pos.x as usize] = '.';
    }
}

fn get_accessible_count(matrix: &mut ForkliftMatrix) -> u32 {
    let mut count = 0;
    let mut remove_positions: Vec<Position> = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if *item == '@'
                && is_accessible_by_forklift(
                    matrix,
                    &Position {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                )
            {
                count += 1;
                remove_positions.push(Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    remove_rolls(matrix, remove_positions);

    if count > 0 {
        count + get_accessible_count(matrix)
    } else {
        count
    }
}

fn get_forklift_matrix(lines: Vec<String>) -> ForkliftMatrix {
    let mut forklift_matrix: ForkliftMatrix = Vec::new();

    for line in lines {
        forklift_matrix.push(line.chars().collect());
    }

    forklift_matrix
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let mut forklift_matrix = get_forklift_matrix(lines);
    let count = get_accessible_count(&mut forklift_matrix);

    println!("Accessible rolls count: {}", count);
}
