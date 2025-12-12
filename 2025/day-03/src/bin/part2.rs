use aoc_lib::read_lines;

type BatteryBank = Vec<u32>;

fn get_battery_banks(lines: Vec<String>) -> Vec<BatteryBank> {
    let mut battery_banks: Vec<BatteryBank> = Vec::new();
    for line in lines {
        battery_banks.push(line.chars().map(|b| b.to_digit(10).unwrap()).collect());
    }
    battery_banks
}

fn calculate_joltage(digits: Vec<(u32, usize)>) -> u64 {
    let mut joltage = 0;
    for (i, digit) in digits.iter().enumerate() {
        let joltage_current = 10u64.pow((digits.len() - i - 1) as u32) * digit.0 as u64;
        joltage += joltage_current;
    }
    joltage
}

fn find_largest(start: usize, end: usize, bank: &BatteryBank) -> (u32, usize) {
    let mut largest: u32 = 0;
    let mut index: usize = 0;
    for (i, digit) in bank.iter().enumerate().take(end + 1).skip(start) {
        if *digit > largest {
            largest = *digit;
            index = i;
        }
    }
    (largest, index)
}

fn get_largest_joltage(battery_bank: &BatteryBank, number_of_digits: usize) -> u64 {
    let mut digits: Vec<(u32, usize)> = Vec::new();
    digits.push(find_largest(
        0,
        battery_bank.len() - number_of_digits,
        battery_bank,
    ));
    for i in 1..number_of_digits {
        digits.push(find_largest(
            digits[i - 1].1 + 1,
            battery_bank.len() - number_of_digits + i,
            battery_bank,
        ))
    }
    calculate_joltage(digits)
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let battery_banks: Vec<BatteryBank> = get_battery_banks(lines);

    let mut joltages_sum: u64 = 0;
    for battery_bank in battery_banks {
        joltages_sum += get_largest_joltage(&battery_bank, 12);
    }

    println!("Maximum joltage: {}", joltages_sum);
}
