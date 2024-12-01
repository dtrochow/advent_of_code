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

fn get_list_counts(list: &[i32]) -> Vec<u32> {
    let max_value = *list.iter().max().unwrap();
    let mut list_counts: Vec<u32> = Vec::new();
    /* Push 0 for the zero number count */
    list_counts.push(0);

    for i in 1..max_value + 1 {
        list_counts.push(list.iter().filter(|&n| *n == i).count().try_into().unwrap());
    }

    list_counts
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let (left_list, right_list) = get_lists(&lines);

    let right_counts = get_list_counts(&right_list);

    let mut similarity_score = 0;
    for num in left_list {
        if num <= right_counts.len().try_into().unwrap() {
            similarity_score += num as u32 * right_counts[num as usize];
        }
    }

    println!("Similarity score: {}", similarity_score);
}
