use std::fs::read_to_string;

type Positions = Vec<u32>;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let positions: Positions = get_positions(lines.first().unwrap());
    let highest_pos: u32 = *positions.iter().max().unwrap();

    let mut least_fuel: (u32, u32) = (u32::MAX, 0);
    for pos in 0..=highest_pos {
        let fuel_needed = calculate_fuel_needed(&positions, pos);
        if fuel_needed < least_fuel.0 {
            least_fuel.0 = fuel_needed;
            least_fuel.1 = pos;
        }
    }
    println!(
        "Least fuel: {} | At position: {}",
        least_fuel.0, least_fuel.1
    );
}

fn calculate_fuel_needed(positions: &Positions, align_pos: u32) -> u32 {
    let mut fuel_needed: u32 = 0;
    for pos in positions {
        fuel_needed += (align_pos as i32 - *pos as i32).unsigned_abs();
    }
    fuel_needed
}

fn get_positions(line: &str) -> Positions {
    line.split(',').map(|x| x.parse::<u32>().unwrap()).collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
