use aoc_lib::read_lines;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    clicks: i32,
}

fn get_all_rotations(lines: Vec<String>) -> Vec<Rotation> {
    let mut rotations = Vec::new();
    for line in lines {
        let direction = match &line[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };

        let clicks: i32 = line[1..].parse().expect("Invalid number");

        rotations.push(Rotation { direction, clicks });
    }

    rotations
}

fn rotate_dial(rotations: Vec<Rotation>, starting_point: i32) -> u32 {
    let mut number_of_zero_hit: u32 = 0;
    let mut current_position: i32 = starting_point;

    for rotation in rotations {
        println!("Current position: {}", current_position);
        println!(
            "Rotating {:?} for {} clicks",
            rotation.direction, rotation.clicks
        );

        let clicks: i32 = rotation.clicks;
        let full_rotations: u32 = (clicks / 100).try_into().unwrap();
        println!("Full rotations: {}", full_rotations);
        number_of_zero_hit += full_rotations;

        match rotation.direction {
            Direction::Left => {
                let diff = current_position - (clicks % 100);
                if diff < 0 {
                    if current_position != 0 {
                        number_of_zero_hit += 1;
                        println!("Zero hit left! {}", number_of_zero_hit);
                    }
                    current_position = 100 - diff.abs();
                } else {
                    current_position = diff;
                }
            }
            Direction::Right => {
                let diff = current_position + (clicks % 100);

                if diff > 99 {
                    current_position = diff - 100;
                    if current_position != 0 {
                        number_of_zero_hit += 1;
                        println!("Zero hit right! {}", number_of_zero_hit);
                    }
                } else {
                    current_position = diff;
                }
            }
        }

        if current_position == 0 {
            number_of_zero_hit += 1;
            println!("Zero hit! {}", number_of_zero_hit);
        }

        if !(0..=99).contains(&current_position) {
            println!("ERROR: {:?}", rotation);
            return 0;
        }
        println!("Current position: {}\n", current_position);
    }
    number_of_zero_hit
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let rotations = get_all_rotations(lines.clone());

    let starting_point: i32 = 50;
    let number_of_zero_hit: u32 = rotate_dial(rotations, starting_point);

    println!("Number of zero hits: {}", number_of_zero_hit);
}
