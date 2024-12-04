use aoc_lib::{read_lines, rotate_matrix_90_clockwise};

type XmasMatrix = Vec<Vec<char>>;

fn get_xmas_matrix(lines: Vec<String>) -> XmasMatrix {
    let mut xmas_matrix: XmasMatrix = Vec::new();

    for line in lines {
        xmas_matrix.push(line.chars().collect());
    }

    xmas_matrix
}

fn is_x_mas(x: usize, y: usize, xmas_matrix: &XmasMatrix) -> bool {
    let size = xmas_matrix.len();

    if !(x < 1 || y < 1 || y > (size - 2) || x > (size - 2)) {
        let mut mas_matrix = vec![
            vec![xmas_matrix[x - 1][y - 1], xmas_matrix[x - 1][y + 1]],
            vec![xmas_matrix[x + 1][y - 1], xmas_matrix[x + 1][y + 1]],
        ];

        for _ in 0..4 {
            if mas_matrix[0] == ['M', 'S'] && mas_matrix[1] == ['M', 'S'] {
                return true;
            }
            mas_matrix = rotate_matrix_90_clockwise(mas_matrix);
        }
    }

    false
}

fn find_all_x_mas(xmas_matrix: XmasMatrix) -> u32 {
    let mut x_mas_count = 0;

    for (x, row) in xmas_matrix.iter().enumerate() {
        for (y, ch) in row.iter().enumerate() {
            if *ch == 'A' && is_x_mas(x, y, &xmas_matrix) {
                x_mas_count += 1;
            }
        }
    }

    x_mas_count
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let xmas_matrix = get_xmas_matrix(lines);
    let xmas_count = find_all_x_mas(xmas_matrix);

    println!("X-MAS count: {}", xmas_count);
}
