use std::fs::read_to_string;
use std::collections::HashMap;

type Position = (usize, usize);

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let mut parts: HashMap<Position, Vec<u32>> = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        let found_parts_with_star = find_parts_with_star(line, index, &lines);
        // Extend the map with keeping already existing values in original map
        for (key, values) in found_parts_with_star {
            parts.entry(key)
                 .and_modify(|existing_val| existing_val.extend(&values))
                 .or_insert(values);
        }
    }

    let mut gear_ratios: Vec<u32> = Vec::<u32>::new();
    for (key, part_numbers) in parts {
        println!("Key: {:?} Value: {:?}", key, part_numbers);
        if part_numbers.len() == 2 {
            gear_ratios.push(part_numbers.first().unwrap() * part_numbers.last().unwrap());
        }
    }
    let sum = gear_ratios.iter().sum::<u32>();
    println!("The sum is: {}", sum);
}

fn find_parts_with_star(line: &String, line_index: usize, lines: &Vec<String>) -> HashMap<Position, Vec<u32>> {
    let mut num_str: String = String::new();
    let mut parts: HashMap<Position, Vec<u32>> = HashMap::new();

    for (ch_index, ch) in line.chars().enumerate() {
        if ch.to_digit(10).is_some() {
            num_str.push(ch);
        }
        if ((!ch.to_digit(10).is_some()) || (ch_index == (line.len()-1))) && (num_str.len() > 0) {
            let result = is_part_number_with_star(lines, line_index, ch_index, num_str.len());
            match result {
                Some(coordinates) => {
                    for position in coordinates {
                        println!("Star found around {} number - position {:?}", num_str, position);
                        parts.entry(position).or_insert_with(Vec::new).push(num_str.parse().unwrap());
                    }
                },
                None => (),
            }
            num_str.clear();
        }
    }

    parts
}

// It cannot return early, because all the stars must be found
fn is_part_number_with_star(lines: &Vec<String>, line_index: usize, char_index: usize, num_digits_count: usize) -> Option<Vec<Position>> {
    const GEAR_SIGN: char = '*';

    let mut star_positions: Vec<Position> = Vec::new();
    let mut signs_to_check_horizontally: usize = num_digits_count;
    /*
    . . . . . . . .
    . . . 1 2 3 X .
    . . . . . . . .
     */
    if !lines[line_index].chars().nth(char_index).unwrap().is_digit(10) {
        if lines[line_index].chars().nth(char_index).unwrap() == GEAR_SIGN {
            star_positions.push((line_index, char_index));
        }
        signs_to_check_horizontally += 1;
    }
    /*
    . . . . . . . .
    . . X 1 2 3 . .
    . . . . . . . .
     */
    if char_index > num_digits_count {
        let mut index = char_index - num_digits_count;
        if lines[line_index].chars().nth(index).unwrap().is_digit(10) {
            index -= 1;
        }
        if lines[line_index].chars().nth(index).unwrap() == GEAR_SIGN {
            star_positions.push((line_index, index));
        }
        signs_to_check_horizontally += 1;
    }
    /*
    . . X X X X X .
    . . . 1 2 3 . .
    . . . . . . . .
     */
    if line_index > 0 {
        for i in 0..signs_to_check_horizontally {
            if lines[line_index - 1].chars().nth(char_index-i).unwrap() == GEAR_SIGN {
                star_positions.push((line_index - 1, char_index-i));
            }
        }
    }
    /*
    . . . . . . . .
    . . . 1 2 3 . .
    . . X X X X X .
     */
    if line_index < (lines.len() - 1) {
        for i in 0..signs_to_check_horizontally {
            if lines[line_index + 1].chars().nth(char_index-i).unwrap() == GEAR_SIGN {
                star_positions.push((line_index + 1, char_index-i));
            }
        }
    }

    if star_positions.len() > 0 {
        return Some(star_positions);
    }

    None
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
