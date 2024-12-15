use aoc_lib::{is_in_2d_map, read_lines, Axis, Direction, Point, DIRECTIONS};

type Garden = Vec<Vec<char>>;

/*
    y ->
  x . . .
  | . . .
  v . . .
*/

struct Field {
    area: u64,
    perimeter: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Edge {
    pos: Point,
    dir: Direction,
    id: u64,
}

fn get_garden_layout(lines: Vec<String>) -> Garden {
    lines.iter().map(|row| row.chars().collect()).collect()
}

fn get_field_points(
    garden: &Garden,
    start: &Point,
    registered: &mut Vec<Point>,
) -> Option<(Vec<Point>, char)> {
    if registered.contains(start) {
        return None;
    }

    let mut stack: Vec<Point> = vec![*start];
    let mut field_points: Vec<Point> = Vec::new();
    let plant_kind = garden[start.x as usize][start.y as usize];

    while let Some(position) = stack.pop() {
        if field_points.contains(&position) {
            continue;
        }
        for direction in DIRECTIONS {
            let neighbor_pos = position + direction;
            if is_in_2d_map(garden, &neighbor_pos)
                && !field_points.contains(&neighbor_pos)
                && garden[neighbor_pos.x as usize][neighbor_pos.y as usize] == plant_kind
            {
                stack.push(neighbor_pos);
            }
        }
        field_points.push(position);
    }
    registered.extend(field_points.clone());

    Some((field_points, plant_kind))
}

fn get_direction(point: &Point) -> Option<Direction> {
    if (*point == Point { x: 1, y: 0 }) {
        Some(Direction::Down)
    } else if (*point == Point { x: 0, y: 1 }) {
        Some(Direction::Right)
    } else if (*point == Point { x: -1, y: 0 }) {
        Some(Direction::Up)
    } else if (*point == Point { x: 0, y: -1 }) {
        Some(Direction::Left)
    } else {
        None
    }
}

fn get_edges_with(
    edges: &[Edge],
    dir: Direction,
    x: Option<i64>,
    y: Option<i64>,
) -> Option<Vec<Edge>> {
    if let Some(x) = x {
        Some(
            edges
                .iter()
                .filter(|e| e.pos.x == x && e.dir == dir)
                .cloned()
                .collect(),
        )
    } else {
        y.map(|y| {
            edges
                .iter()
                .filter(|e| e.pos.y == y && e.dir == dir)
                .cloned()
                .collect()
        })
    }
}

fn count_edges(edges: &mut [Edge], axis: Axis) -> u64 {
    if edges.is_empty() {
        return 0;
    }
    let mut edges_cnt = 0;
    let mut prev;
    match axis {
        Axis::X => {
            edges_cnt += 1;
            edges.sort_by_key(|e| e.pos.y);
            prev = *edges.first().unwrap();
            for edge in edges {
                if (edge.pos.y - prev.pos.y).abs() > 1 {
                    edges_cnt += 1;
                }
                prev = *edge;
            }
        }
        Axis::Y => {
            edges.sort_by_key(|e| e.pos.x);
            prev = *edges.first().unwrap();
            edges_cnt += 1;
            for edge in edges {
                if (edge.pos.x - prev.pos.x).abs() > 1 {
                    edges_cnt += 1;
                }
                prev = *edge;
            }
        }
    }
    edges_cnt
}

fn get_common_edges_cnt(edges: &[Edge], garden: &Garden) -> u64 {
    let mut edges_cnt = 0;
    let mut f_edges;

    for x in 0..garden.len() {
        f_edges = get_edges_with(edges, Direction::Up, Some(x as i64), None).unwrap();
        edges_cnt += count_edges(&mut f_edges, Axis::X);

        f_edges = get_edges_with(edges, Direction::Down, Some(x as i64), None).unwrap();
        edges_cnt += count_edges(&mut f_edges, Axis::X);
    }

    for y in 0..garden.first().unwrap().len() {
        f_edges = get_edges_with(edges, Direction::Left, None, Some(y as i64)).unwrap();
        edges_cnt += count_edges(&mut f_edges, Axis::Y);

        f_edges = get_edges_with(edges, Direction::Right, None, Some(y as i64)).unwrap();
        edges_cnt += count_edges(&mut f_edges, Axis::Y);
    }

    edges_cnt
}

fn get_field_edges_count(garden: &Garden, field_points: &Vec<Point>, plant_kind: &char) -> u64 {
    let mut edges: Vec<Edge> = Vec::new();

    for plant_pos in field_points {
        for direction in DIRECTIONS {
            let neighbor_pos = direction + *plant_pos;
            if !is_in_2d_map(garden, &neighbor_pos)
                || garden[neighbor_pos.x as usize][neighbor_pos.y as usize] != *plant_kind
            {
                edges.push(Edge {
                    pos: *plant_pos,
                    dir: get_direction(&direction).unwrap(),
                    id: 0,
                })
            }
        }
    }

    get_common_edges_cnt(&edges, garden)
}

fn calculate_price(plant_fields: Vec<Field>) -> u64 {
    plant_fields
        .iter()
        .map(|field| field.area * field.perimeter)
        .sum()
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let garden = get_garden_layout(lines);

    let mut registered_plants: Vec<Point> = Vec::new();
    let mut plant_fields: Vec<Field> = Vec::new();

    for (x, row) in garden.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let plant_pos = Point {
                x: x as i64,
                y: y as i64,
            };
            let (field_points, plant_kind) =
                match get_field_points(&garden, &plant_pos, &mut registered_plants) {
                    None => {
                        continue;
                    }
                    Some(points) => points,
                };
            plant_fields.push(Field {
                area: field_points.len() as u64,
                perimeter: get_field_edges_count(&garden, &field_points, &plant_kind),
            });
        }
    }

    let price = calculate_price(plant_fields);
    println!("Total price: {}", price);
}
