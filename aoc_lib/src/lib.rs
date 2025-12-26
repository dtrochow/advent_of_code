use std::fs::read_to_string;
use std::ops::{Add, Sub};
use strum_macros::EnumIter;

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn point(&self) -> Point {
        /*
           x ->
         y . . .
         | . . .
         v . . .
        */
        match self {
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, EnumIter, Clone)]
pub enum QuadrantId {
    I,
    II,
    III,
    IV,
}

/*
   I  | II
 ---------
  III | IV
*/
pub fn get_quadrant_borders(q_id: QuadrantId, field_size: &Point) -> Borders {
    match q_id {
        QuadrantId::I => Borders {
            up: 0,
            down: (field_size.y / 2) - 1,
            left: 0,
            right: (field_size.x / 2) - 1,
        },
        QuadrantId::II => Borders {
            up: 0,
            down: (field_size.y / 2) - 1,
            left: (field_size.x / 2) + 1,
            right: field_size.x - 1,
        },
        QuadrantId::III => Borders {
            up: (field_size.y / 2) + 1,
            down: field_size.y - 1,
            left: 0,
            right: (field_size.x / 2) - 1,
        },
        QuadrantId::IV => Borders {
            up: (field_size.y / 2) + 1,
            down: field_size.y - 1,
            left: (field_size.x / 2) + 1,
            right: field_size.x - 1,
        },
    }
}

#[derive(Debug)]
pub struct Borders {
    pub up: i64,
    pub down: i64,
    pub left: i64,
    pub right: i64,
}

impl Borders {
    pub fn is_within_borders(&self, point: &Point) -> bool {
        point.x >= self.left && point.x <= self.right && point.y >= self.up && point.y <= self.down
    }
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

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    pub fn is_symmetrical_in_x_axis(&self, point: &Point, symmetry_axis: usize) -> bool {
        self.y == point.y
            && (self.x - symmetry_axis as i64).abs() == (point.x - symmetry_axis as i64).abs()
            && self != point
    }
}

/**
 *   y ->
 *  x . . .
 *  | . . .
 *  v . . .
 */
pub fn is_in_2d_map<T>(v: &[Vec<T>], point: &Point) -> bool {
    point.x >= 0
        && point.x < v.len() as i64
        && point.y >= 0
        && point.y < v.first().unwrap().len() as i64
}

/**
 *   x ->
 *  y . . .
 *  | . . .
 *  v . . .
 */
pub fn is_in_2d_map2<T>(v: &[Vec<T>], point: &Point) -> bool {
    point.y >= 0
        && point.y < v.len() as i64
        && point.x >= 0
        && point.x < v.first().unwrap().len() as i64
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
                    Some(Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    })
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

    #[test]
    fn test_is_in_2d_map() {
        let map = vec![vec![0; 5]; 8];
        assert!(is_in_2d_map(&map, &Point { x: 7, y: 4 }));
        assert!(!is_in_2d_map(&map, &Point { x: 7, y: 5 }));
        assert!(!is_in_2d_map(&map, &Point { x: 8, y: 4 }));

        let map2 = vec![vec![0; 10]; 9];
        assert!(is_in_2d_map(&map2, &Point { x: 8, y: 9 }));
        assert!(!is_in_2d_map(&map2, &Point { x: 9, y: 9 }));
        assert!(!is_in_2d_map(&map2, &Point { x: 8, y: 10 }));
    }

    #[test]
    fn test_is_in_2d_map2() {
        let map = vec![vec![0; 5]; 8];
        assert!(is_in_2d_map2(&map, &Point { x: 4, y: 7 }));
        assert!(!is_in_2d_map2(&map, &Point { x: 5, y: 7 }));
        assert!(!is_in_2d_map2(&map, &Point { x: 4, y: 8 }));

        let map2 = vec![vec![0; 10]; 9];
        assert!(is_in_2d_map2(&map2, &Point { x: 9, y: 8 }));
        assert!(!is_in_2d_map2(&map2, &Point { x: 9, y: 9 }));
        assert!(!is_in_2d_map2(&map2, &Point { x: 10, y: 8 }));
    }
}
