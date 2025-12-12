use aoc_lib::read_lines;

type BatteryBank = Vec<u32>;

fn get_battery_banks(lines: Vec<String>) -> Vec<BatteryBank> {
    let mut battery_banks: Vec<BatteryBank> = Vec::new();
    for line in lines {
        battery_banks.push(line.chars().map(|b| b.to_digit(10).unwrap()).collect());
    }
    battery_banks
}

fn find_tens_digit(battery_bank: &BatteryBank) -> (u32, usize) {
    let mut max = 0;
    let mut index = 0;
    for (i, battery) in battery_bank.iter().enumerate().take(battery_bank.len() - 1) {
        if *battery > max {
            max = *battery;
            index = i;
        }
    }

    (max, index)
}

fn find_max_units_digit(battery_bank: &BatteryBank, tens_index: usize) -> u32 {
    let mut max = 0;
    for bank in battery_bank.iter().skip(tens_index + 1) {
        if *bank > max {
            max = *bank;
        }
    }
    max
}

fn get_largest_joltage(battery_bank: &BatteryBank) -> u32 {
    let (tens, tens_index) = find_tens_digit(battery_bank);
    let units = find_max_units_digit(battery_bank, tens_index);

    10 * tens + units
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let battery_banks: Vec<BatteryBank> = get_battery_banks(lines);

    let mut joltages_sum: u32 = 0;
    for battery_bank in battery_banks {
        joltages_sum += get_largest_joltage(&battery_bank);
    }

    println!("Maximum joltage: {}", joltages_sum);
}
