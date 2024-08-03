use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Position {
    horizontal: i64,
    depth: i64,
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    value: i64,
}

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
    Unknown,
}

impl FromStr for Direction {
    type Err = Direction;

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(Direction::Unknown),
        }
    }
}

fn main() {
    let mut position: Position = Position {
        horizontal: 0,
        depth: 0,
    };

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    for line in lines {
        move_submarine(line, &mut position);
    }

    println!("Position: {:?}", position);
    println!(
        "Multiplication result: {}",
        position.horizontal * position.depth
    );
}

fn move_submarine(line: String, position: &mut Position) {
    let instruction: Instruction = Instruction {
        dir: line.split(' ').next().unwrap().parse().unwrap(),
        value: line.split(' ').last().unwrap().parse().unwrap(),
    };
    match instruction.dir {
        Direction::Up => position.depth -= instruction.value,
        Direction::Down => position.depth += instruction.value,
        Direction::Forward => position.horizontal += instruction.value,
        _ => println!("Unknown direction!"),
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
