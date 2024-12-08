use aoc_lib::read_lines;

#[derive(Debug, Clone, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

type Map = Vec<Vec<char>>;

const EMPTY: char = '.';

fn get_map(lines: Vec<String>) -> Map {
    lines.iter().map(|row| row.chars().collect()).collect()
}

fn find_other_antennas(map: &Map, origin_pos: &Point, antenna_type: &char) -> Vec<Point> {
    let mut antennas = Vec::new();
    for (x, row) in map.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if (Point {
                x: y as i64,
                y: y as i64,
            }) == *origin_pos
            {
                continue;
            }
            if value == antenna_type {
                antennas.push(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    antennas
}

fn calculate_antenna_vectors(
    origin_pos: &Point,
    other_antennas: Vec<Point>,
) -> Vec<(Point, Vector)> {
    let mut vectors = Vec::new();
    for antenna_pos in other_antennas {
        if *origin_pos != antenna_pos {
            vectors.push((
                antenna_pos.clone(),
                Vector {
                    x: antenna_pos.x - origin_pos.x,
                    y: antenna_pos.y - origin_pos.y,
                },
            ));
        }
    }
    vectors
}

fn is_in_map(map: &Map, pos: &Point) -> bool {
    pos.x >= 0
        && pos.x < map.len().try_into().unwrap()
        && pos.y >= 0
        && pos.y < map[0].len().try_into().unwrap()
}

fn save_antinode(antinode_positions: &mut Vec<Point>, antinode_pos: Point) {
    if !antinode_positions.contains(&antinode_pos) {
        antinode_positions.push(antinode_pos);
    }
}

fn check_all_antinode_at_pos(
    map: &Map,
    antinode_positions: &mut Vec<Point>,
    pos: Point,
    antenna: &char,
) {
    /* Find all other antennas of the same type */
    let other_antennas = find_other_antennas(map, &pos, antenna);

    /* Calculate the vector for each found antenna */
    let vectors = calculate_antenna_vectors(&pos, other_antennas);

    /* Check each vector for antinode
    - one in a negative direction
    - one in a positive direction but multiplied by 2 */
    /* Mark each antinode in the antinode_map */
    for vector in vectors {
        let first_antinode = Point {
            x: pos.x - vector.1.x,
            y: pos.y - vector.1.y,
        };
        if is_in_map(map, &first_antinode) {
            save_antinode(antinode_positions, first_antinode);
        }
        let second_antinode = Point {
            x: vector.0.x + vector.1.x,
            y: vector.0.y + vector.1.y,
        };
        if is_in_map(map, &second_antinode) {
            save_antinode(antinode_positions, second_antinode);
        }
    }
}

fn get_antinode_count(map: &Map) -> u64 {
    let mut antinode_positions: Vec<Point> = Vec::new();
    for (x, row) in map.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if *value != EMPTY {
                check_all_antinode_at_pos(
                    map,
                    &mut antinode_positions,
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    value,
                );
            }
        }
    }
    antinode_positions.len().try_into().unwrap()
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let map = get_map(lines);
    let antinode_count = get_antinode_count(&map);

    println!("Antinode count: {}", antinode_count);
}
