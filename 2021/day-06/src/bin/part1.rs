use std::fs::read_to_string;

type Fishes = Vec<u8>;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut fishes: Fishes = get_initial_fishes(lines.first().unwrap());
    run_simulation(&mut fishes, 80);
    println!("Fishes count: {}", fishes.len());
}

fn run_simulation(fishes: &mut Fishes, days: u8) {
    for _ in 0..days {
        let mut fish_to_spawn: u64 = 0;
        for fish in &mut fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                fish_to_spawn += 1;
                continue;
            }
            *fish -= 1;
        }
        for _ in 0..fish_to_spawn {
            fishes.push(8);
        }
    }
}

fn get_initial_fishes(line: &str) -> Fishes {
    line.split(',').map(|x| x.parse::<u8>().unwrap()).collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
