use std::fs::read_to_string;

#[derive(Debug, Clone, Default)]
struct BitCount {
    ones: u64,
    zeros: u64,
}

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let bit_counts: Vec<BitCount> = get_bits_count(&lines);
    let gamma_rate: u64 = get_gamma_rate(&bit_counts);
    let mask_length: usize = lines.first().unwrap().len();
    let sigma_rate: u64 = get_sigma_rage(&gamma_rate, mask_length);

    println!("Gamma rate: {}", gamma_rate);
    println!("Sigma rate: {}", sigma_rate);
    println!("Power consumption: {}", gamma_rate * sigma_rate);
}

fn get_sigma_rage(gamma_rate: &u64, mask_length: usize) -> u64 {
    let mask = (1 << mask_length) - 1;
    !gamma_rate & mask
}

fn get_gamma_rate(bit_counts: &Vec<BitCount>) -> u64 {
    let mut gamma_rate: String = String::new();
    for bit_count in bit_counts {
        if bit_count.ones > bit_count.zeros {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
    }

    match u64::from_str_radix(&gamma_rate, 2) {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse gamma rate value!");
            0
        }
    }
}

fn get_bits_count(lines: &Vec<String>) -> Vec<BitCount> {
    let mut bit_counts: Vec<BitCount> = vec![BitCount::default(); lines.first().unwrap().len()];
    for line in lines {
        for (index, bit) in line.chars().enumerate() {
            match bit {
                '0' => bit_counts[index].zeros += 1,
                '1' => bit_counts[index].ones += 1,
                _ => println!("Error - Unexpected bit value!"),
            }
        }
    }
    bit_counts
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
