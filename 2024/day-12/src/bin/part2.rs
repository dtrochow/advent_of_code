use aoc_lib::{is_in_2d_map, read_lines, Point, DIRECTIONS};

type Garden = Vec<Vec<char>>;

struct Field {
    area: u64,
    perimeter: u64,
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

    let mut stack: Vec<Point> = vec![start.clone()];
    let mut field_points: Vec<Point> = Vec::new();
    let plant_kind = garden[start.x as usize][start.y as usize];

    while let Some(position) = stack.pop() {
        if field_points.contains(&position) {
            continue;
        }
        for direction in DIRECTIONS {
            let neighbor_pos = position.clone() + direction;
            if is_in_2d_map(garden, &neighbor_pos)
                && !field_points.contains(&neighbor_pos)
                && garden[neighbor_pos.x as usize][neighbor_pos.y as usize] == plant_kind
            {
                stack.push(neighbor_pos);
            }
        }
        field_points.push(position.clone());
    }
    registered.extend(field_points.clone());

    Some((field_points, plant_kind))
}

fn get_field_perimeter(garden: &Garden, field_points: &Vec<Point>, plant_kind: &char) -> u64 {
    let mut fence_length = 0;
    for flower_pos in field_points {
        for direction in DIRECTIONS {
            let neighbor_pos = direction + flower_pos.clone();
            if !is_in_2d_map(garden, &neighbor_pos)
                || garden[neighbor_pos.x as usize][neighbor_pos.y as usize] != *plant_kind
            {
                fence_length += 1;
            }
        }
    }
    fence_length
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
                perimeter: get_field_perimeter(&garden, &field_points, &plant_kind),
            });
        }
    }

    let price = calculate_price(plant_fields);
    println!("Total price: {}", price);
}
