use aoc_lib::{read_lines, transpose_matrix};

type XmasMatrix = Vec<Vec<char>>;
type XmasRow = Vec<char>;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

fn get_xmas_matrix(lines: Vec<String>) -> XmasMatrix {
    let mut xmas_matrix: XmasMatrix = Vec::new();

    for line in lines {
        xmas_matrix.push(line.chars().collect());
    }

    xmas_matrix
}

fn is_horizontal_xmas(start_index: usize, row: &XmasRow, dir: Direction) -> bool {
    if row.len() - start_index < 4 {
        return false;
    }
    let row_slice = &row[start_index..start_index + 4];

    match dir {
        Direction::Right => row_slice == ['X', 'M', 'A', 'S'],
        Direction::Left => row_slice == ['S', 'A', 'M', 'X'],
    }
}

fn find_horizontal(xmas_matrix: &XmasMatrix) -> u32 {
    let mut horizontal_count = 0;

    for row in xmas_matrix {
        for i in 0..row.len() {
            if is_horizontal_xmas(i, row, Direction::Right) {
                horizontal_count += 1;
            }
            if is_horizontal_xmas(i, row, Direction::Left) {
                horizontal_count += 1;
            }
        }
    }

    horizontal_count
}

fn find_vertical(xmas_matrix: &XmasMatrix) -> u32 {
    let mut vertical_count = 0;

    let transposed_matrix: XmasMatrix = transpose_matrix(xmas_matrix.to_vec());
    for row in transposed_matrix {
        for i in 0..row.len() {
            if is_horizontal_xmas(i, &row, Direction::Right) {
                vertical_count += 1;
            }
            if is_horizontal_xmas(i, &row, Direction::Left) {
                vertical_count += 1;
            }
        }
    }

    vertical_count
}

fn get_diagonal_count(x: usize, y: usize, xmas_matrix: &XmasMatrix) -> u32 {
    let size = xmas_matrix.len();
    let correct_word: Vec<char> = vec!['M', 'A', 'S'];

    let mut diagonal_count = 0;
    let mut curr_word: Vec<char> = vec!['0'; 3];

    /* Left Up */
    if x >= 3 && y >= 3 {
        for i in 0..correct_word.len() {
            curr_word[i] = xmas_matrix[x - i - 1][y - i - 1];
        }
        if curr_word == correct_word {
            diagonal_count += 1;
        }
    }

    /* Right Up */
    if x >= 3 && y <= (size - 4) {
        for i in 0..correct_word.len() {
            curr_word[i] = xmas_matrix[x - i - 1][y + i + 1];
        }
        if curr_word == correct_word {
            diagonal_count += 1;
        }
    }

    /* Left Down */
    if x <= (size - 4) && y >= 3 {
        for i in 0..correct_word.len() {
            curr_word[i] = xmas_matrix[x + i + 1][y - i - 1];
        }
        if curr_word == correct_word {
            diagonal_count += 1;
        }
    }

    /* Right Down */
    if x <= (size - 4) && y <= (size - 4) {
        for i in 0..correct_word.len() {
            curr_word[i] = xmas_matrix[x + i + 1][y + i + 1];
        }
        if curr_word == correct_word {
            diagonal_count += 1;
        }
    }

    diagonal_count
}

fn find_diagonal(xmas_matrix: &XmasMatrix) -> u32 {
    let mut diagonal_count = 0;

    for (x, row) in xmas_matrix.iter().enumerate() {
        for (y, ch) in row.iter().enumerate() {
            if *ch == 'X' {
                diagonal_count += get_diagonal_count(x, y, xmas_matrix);
            }
        }
    }

    diagonal_count
}

fn find_all_xmas(xmas_matrix: XmasMatrix) -> u32 {
    find_horizontal(&xmas_matrix) + find_vertical(&xmas_matrix) + find_diagonal(&xmas_matrix)
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let xmas_matrix = get_xmas_matrix(lines);
    let xmas_count = find_all_xmas(xmas_matrix);

    println!("XMAS count: {}", xmas_count);
}
