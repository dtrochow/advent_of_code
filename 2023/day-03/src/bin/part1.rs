use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let mut sum: u32 = 0;
    for (index, line) in lines.iter().enumerate() {
        sum += find_numbers(line, index, &lines).iter().sum::<u32>();
    }
    println!("The sum is: {}", sum);
}

fn find_numbers(line: &String, line_index: usize, lines: &Vec<String>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::<u32>::new();
    let mut num_str: String = String::new();

    for (ch_index, ch) in line.chars().enumerate() {
        if ch.to_digit(10).is_some() {
            num_str.push(ch);
        }
        if ((!ch.to_digit(10).is_some()) || (ch_index == (line.len()-1))) && (num_str.len() > 0) {
            if is_part_number(lines, line_index, ch_index, num_str.len()) {
                numbers.push(num_str.parse().unwrap());
            }
            num_str.clear();
        }
    }

    numbers
}

fn is_part_number(lines: &Vec<String>, line_index: usize, char_index: usize, num_digits_count: usize) -> bool {
    const DOT_SIGN: char = '.';
    let mut signs_to_check_horizontally: usize = num_digits_count;
    /*
    . . . . . . . .
    . . . 1 2 3 X .
    . . . . . . . .
     */
    if !lines[line_index].chars().nth(char_index).unwrap().is_digit(10) {
        if lines[line_index].chars().nth(char_index).unwrap() != DOT_SIGN {
            return true;
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
        if lines[line_index].chars().nth(index).unwrap() != DOT_SIGN {
            return true;
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
            if lines[line_index - 1].chars().nth(char_index-i).unwrap() != DOT_SIGN {
                return true;
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
            if lines[line_index + 1].chars().nth(char_index-i).unwrap() != DOT_SIGN {
                return true;
            }
        }
    }
    false
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
