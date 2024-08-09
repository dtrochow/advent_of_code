use std::fs::read_to_string;

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    start: Pos,
    end: Pos,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

type OceanFloor = Vec<Vec<u32>>;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut cart_lines: Vec<Line> = Vec::new();
    for line in lines {
        cart_lines.push(get_line(line));
    }

    let (x_size, y_size) = find_ocean_floor_size(&cart_lines);
    let mut ocean_floor: OceanFloor = vec![vec![0; x_size]; y_size];
    for line in cart_lines {
        draw_line(&mut ocean_floor, line);
    }
    let mut overlapping_count: u32 = 0;
    for row in &ocean_floor {
        for field in row {
            if *field >= 2 {
                overlapping_count += 1;
            }
        }
    }
    println!("Overlapping count: {}", overlapping_count);
}

fn find_ocean_floor_size(cart_lines: &Vec<Line>) -> (usize, usize) {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for line in cart_lines {
        if line.start.x > max_x {
            max_x = line.start.x;
        } else if line.end.x > max_x {
            max_x = line.end.x;
        }
        if line.start.y > max_y {
            max_y = line.start.y;
        } else if line.end.y > max_y {
            max_y = line.end.y;
        }
    }
    (max_x + 1, max_y + 1)
}

fn draw_line(ocean_floor: &mut OceanFloor, line: Line) {
    if line.is_vertical() {
        if line.start.y > line.end.y {
            for y in line.end.y..=line.start.y {
                ocean_floor[y][line.start.x] += 1;
            }
        } else {
            for y in line.start.y..=line.end.y {
                ocean_floor[y][line.start.x] += 1;
            }
        }
    } else if line.is_horizontal() {
        if line.start.x > line.end.x {
            for x in line.end.x..=line.start.x {
                ocean_floor[line.start.y][x] += 1;
            }
        } else {
            for x in line.start.x..=line.end.x {
                ocean_floor[line.start.y][x] += 1;
            }
        }
    } else {
        println!("[INFO] Diagonal line");
    }
}

fn get_line(line: String) -> Line {
    let start_pos_str = line.split(" -> ").next().unwrap();
    let start_pos = Pos {
        x: start_pos_str.split(',').next().unwrap().parse().unwrap(),
        y: start_pos_str.split(',').last().unwrap().parse().unwrap(),
    };
    let end_pos_str = line.split(" -> ").last().unwrap();
    let end_pos = Pos {
        x: end_pos_str.split(',').next().unwrap().parse().unwrap(),
        y: end_pos_str.split(',').last().unwrap().parse().unwrap(),
    };
    Line {
        start: start_pos,
        end: end_pos,
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
