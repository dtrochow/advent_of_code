use aoc_lib::read_lines;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn get_points(lines: Vec<String>) -> Vec<Point> {
    lines
        .iter()
        .map(|l| {
            let p: Vec<i64> = l.split(',').map(|n| n.parse().unwrap()).collect();
            Point {
                x: p[0],
                y: p[1],
                z: p[2],
            }
        })
        .collect()
}

fn calculate_distance(first: &Point, second: &Point) -> f64 {
    (((second.x - first.x).pow(2) + (second.y - first.y).pow(2) + (second.z - first.z).pow(2))
        as f64)
        .sqrt()
}

fn get_distances(points: Vec<Point>) -> HashMap<(Point, Point), f64> {
    let mut distances: HashMap<(Point, Point), f64> = HashMap::new();
    for point_1 in &points {
        for point_2 in &points {
            let distance = calculate_distance(point_1, point_2);
            if 0.0 != distance && !distances.contains_key(&(*point_2, *point_1)) {
                distances.insert((*point_1, *point_2), distance);
            }
        }
    }
    distances
}

enum CircuitCase {
    BothTheSame,
    BothDifferent,
    Single,
}

fn find_circuit(
    p1: &Point,
    p2: &Point,
    circuits: &[Vec<Point>],
) -> Option<(CircuitCase, Vec<Option<usize>>)> {
    let mut p1_desc: Option<usize> = None;
    let mut p2_desc: Option<usize> = None;

    for (i, circuit) in circuits.iter().enumerate() {
        if circuit.iter().any(|p| p == p1) {
            p1_desc = Some(i);
        }
        if circuit.iter().any(|p| p == p2) {
            p2_desc = Some(i);
        }
    }

    match (p1_desc, p2_desc) {
        (Some(p1), Some(p2)) => {
            if p1 == p2 {
                Some((CircuitCase::BothTheSame, Vec::new()))
            } else {
                Some((CircuitCase::BothDifferent, vec![Some(p1), Some(p2)]))
            }
        }
        (Some(p1_), None) => Some((CircuitCase::Single, vec![Some(p1_), None])),
        (None, Some(p2_)) => Some((CircuitCase::Single, vec![None, Some(p2_)])),
        (None, None) => None,
    }
}

fn connect_circuits(
    number_of_connections: usize,
    distances: Vec<(Point, Point, f64)>,
) -> Vec<Vec<Point>> {
    let mut connections: Vec<Vec<Point>> = Vec::new();

    for d in distances.iter().take(number_of_connections) {
        let p1 = d.0;
        let p2 = d.1;

        if let Some(id) = find_circuit(&p1, &p2, &connections) {
            // P1 or P2 found in some connection vector
            match id.0 {
                CircuitCase::BothDifferent => {
                    // If P1 AND P2 found in two different vectors -> connect these two vectors
                    let index_1 = id.1[0].unwrap();
                    let index_2 = id.1[1].unwrap();
                    let second_cpy = connections[index_2].clone();
                    connections[index_1].extend(second_cpy);
                    connections.remove(index_2);
                }
                CircuitCase::BothTheSame => {
                    // If P1 AND P2 both found in the same vector - do nothing
                }
                CircuitCase::Single => {
                    // If P1 found then add P2 to the connection vector (if P2 then add P1)
                    let circuits = id.1;
                    if circuits[0].is_some() {
                        let index = circuits[0].unwrap();
                        connections[index].push(p2);
                    } else {
                        let index = circuits[1].unwrap();
                        connections[index].push(p1);
                    }
                }
            }
        } else {
            // If no point found in any connection vector -> just create a new connection
            connections.push(vec![p1, p2]);
        }
    }
    connections
}

fn get_answer(circuits: Vec<Vec<Point>>) -> u64 {
    let mut lengths: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    lengths.sort();
    lengths.reverse();

    (lengths[0] * lengths[1] * lengths[2]) as u64
}

fn cmp_f64(a_: &(Point, Point, f64), b_: &(Point, Point, f64)) -> Ordering {
    let a = a_.2;
    let b = b_.2;
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let points = get_points(lines);

    let distances = get_distances(points);
    let mut distances_v: Vec<(Point, Point, f64)> =
        distances.iter().map(|d| (d.0 .0, d.0 .1, *d.1)).collect();
    distances_v.sort_by(cmp_f64);
    let connections = connect_circuits(1000, distances_v);

    let answer = get_answer(connections);
    println!("\nAnswer: {}", answer);
}
