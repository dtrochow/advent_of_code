use std::fs::read_to_string;

type HeightMatrix = Vec<Vec<u32>>;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let height_matrix: HeightMatrix = read_matrix(lines);
    let low_points: Vec<u32> = find_low_points(&height_matrix);
    let risk_levels: Vec<u32> = low_points.iter().map(|x| x + 1).collect();

    println!("Risk levels sum: {}", risk_levels.iter().sum::<u32>());
}

fn is_low_point(height_matrix: &HeightMatrix, value: u32, pos: (usize, usize)) -> bool {
    let row = pos.0;
    let col = pos.1;
    if col > 0 && height_matrix[row][col - 1] <= value
        || col < (height_matrix[row].len() - 1) && height_matrix[row][col + 1] <= value
        || row > 0 && height_matrix[row - 1][col] <= value
        || row < (height_matrix.len() - 1) && height_matrix[row + 1][col] <= value
    {
        return false;
    }
    true
}

fn find_low_points(height_matrix: &HeightMatrix) -> Vec<u32> {
    let mut low_points: Vec<u32> = Vec::new();
    for (row_i, row) in height_matrix.iter().enumerate() {
        for (col_i, height) in row.iter().enumerate() {
            if is_low_point(height_matrix, *height, (row_i, col_i)) {
                low_points.push(*height);
            }
        }
    }
    low_points
}

fn read_matrix(lines: Vec<String>) -> HeightMatrix {
    let mut height_matrix: HeightMatrix = HeightMatrix::new();
    for line in lines {
        height_matrix.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }
    height_matrix
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
