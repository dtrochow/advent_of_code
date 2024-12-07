use aoc_lib::{find_index, read_lines};

#[derive(Debug)]
struct Rule {
    before: u64,
    after: u64,
}

type Update = Vec<u64>;

fn get_rules(lines: &[String]) -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();
    let sep_index = find_index(lines, &"".to_string()).unwrap();

    for line in &lines[..sep_index] {
        let rule = Rule {
            before: line.split('|').next().unwrap().parse().unwrap(),
            after: line.split('|').last().unwrap().parse().unwrap(),
        };
        rules.push(rule);
    }
    rules
}

fn get_updates(lines: &[String]) -> Vec<Update> {
    let mut updates: Vec<Update> = Vec::new();
    let sep_index = find_index(lines, &"".to_string()).unwrap();

    for line in &lines[sep_index + 1..] {
        updates.push(line.split(',').map(|i| i.parse().unwrap()).collect());
    }

    updates
}

fn get_right_order_updates(updates: &[Update], rules: &Vec<Rule>) -> Vec<usize> {
    let mut right_order_updates: Vec<usize> = Vec::new();

    for (i, update) in updates.iter().enumerate() {
        if is_right_update(update, rules) {
            right_order_updates.push(i)
        }
    }

    right_order_updates
}

fn is_right_update(update: &Update, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if let (Some(before_index), Some(after_index)) = (
            find_index(update, &rule.before),
            find_index(update, &rule.after),
        ) {
            if before_index > after_index {
                return false;
            }
        }
    }
    true
}

fn get_middle_page_numbers_sum(updates: &[Update], indexes: Vec<usize>) -> u64 {
    indexes
        .into_iter()
        .filter_map(|i| updates.get(i))
        .map(get_middle_page_number)
        .sum()
}

fn get_middle_page_number(update: &Update) -> u64 {
    update[update.len() / 2]
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let rules = get_rules(&lines);
    let updates = get_updates(&lines);

    let right_order_updates = get_right_order_updates(&updates, &rules);
    let middle_number_pages_sum = get_middle_page_numbers_sum(&updates, right_order_updates);

    println!("Middle number pages sum: {}", middle_number_pages_sum);
}
