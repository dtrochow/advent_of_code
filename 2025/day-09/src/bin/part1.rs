use aoc_lib::{read_lines, Point};

fn get_points(lines: Vec<String>) -> Vec<Point> {
    lines
        .iter()
        .map(|l| Point {
            x: l.split(',').next().unwrap().parse().unwrap(),
            y: l.split(',').next_back().unwrap().parse().unwrap(),
        })
        .collect()
}

fn calculate_ares(p1: &Point, p2: &Point) -> u64 {
    ((p1.x - p2.x + 1).abs() * (p1.y - p2.y + 1).abs()) as u64
}

fn get_largest_area(points: Vec<Point>) -> u64 {
    let mut max_area: u64 = 0;
    for p1 in &points {
        for p2 in &points {
            if p1 == p2 {
                continue;
            }
            let area = calculate_ares(p1, p2);
            // println!("Points: {:?} {:?} Area: {}", p1, p2, area);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let points: Vec<Point> = get_points(lines);

    let largest_area: u64 = get_largest_area(points);
    println!("Largest area: {}", largest_area);
}
