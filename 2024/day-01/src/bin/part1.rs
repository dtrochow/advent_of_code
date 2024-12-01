use aoc_lib::read_lines;

fn get_lists(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    for line in lines {
        left_list.push(line.split_whitespace().next().unwrap().parse().unwrap());
        right_list.push(line.split_whitespace().last().unwrap().parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let (left_list, right_list) = get_lists(&lines);

    let mut distances_sum = 0;

    for i in 0..lines.len() {
        distances_sum += (left_list[i] - right_list[i]).abs();
    }

    println!("Distances sum: {}", distances_sum);
}
