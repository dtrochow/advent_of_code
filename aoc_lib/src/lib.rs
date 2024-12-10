use std::fs::read_to_string;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn is_in_2d_map<T>(v: &[Vec<T>], point: &Point) -> bool {
    point.x >= 0
        && point.x < v.len() as i64
        && point.y >= 0
        && point.y < v.first().unwrap().len() as i64
}

pub fn find_all_value_positions<T>(v: &[Vec<T>], val: T) -> Vec<Point>
where
    T: PartialEq + Copy,
{
    v.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(move |(y, value)| {
                if val == *value {
                    Some(Point{x: x.try_into().unwrap(), y: y.try_into().unwrap()})
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn transpose_matrix<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn rotate_matrix_90_clockwise<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let vec = transpose_matrix(v);
    vec.into_iter()
        .map(|mut row| {
            row.reverse();
            row
        })
        .collect()
}

pub fn find_index<T: PartialEq>(v: &[T], item: &T) -> Option<usize> {
    v.iter().position(|x| x == item)
}

#[cfg(test)]
mod tests {
    use super::*;
}
