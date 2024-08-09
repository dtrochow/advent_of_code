use std::fs::read_to_string;

type Fishes = Vec<u64>;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut fishes: Fishes = get_initial_fishes(lines.first().unwrap());
    run_simulation(&mut fishes, 256);
    println!("Fishes count: {}", fishes.iter().sum::<u64>());
}

fn run_simulation(fishes: &mut Fishes, days: u16) {
    for _ in 0..days {
        *fishes = [&fishes[1..], &[fishes[0]]].concat();
        fishes[6] += fishes[fishes.len() - 1];
    }
}

fn get_initial_fishes(line: &str) -> Fishes {
    let mut fishes: Fishes = vec![0; 9];
    for timer in line.split(',').map(|x| x.parse::<u32>().unwrap()) {
        fishes[timer as usize] += 1;
    }
    fishes
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
