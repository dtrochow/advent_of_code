use aoc_lib::{find_index, read_lines};

#[derive(Debug, Clone)]
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

fn get_wrong_order_updates(updates: &[Update], rules: &Vec<Rule>) -> Vec<usize> {
    let mut wrong_order_updates: Vec<usize> = Vec::new();

    for (i, update) in updates.iter().enumerate() {
        if !is_right_update(update, rules) {
            wrong_order_updates.push(i)
        }
    }

    wrong_order_updates
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

fn get_applicable_rules(update: &Update, rules: &[Rule]) -> Vec<Rule> {
    rules
        .iter()
        .filter(|rule| {
            find_index(update, &rule.before).is_some() && find_index(update, &rule.after).is_some()
        })
        .cloned()
        .collect()
}

fn fix_single_update(update: &Update, rules: &[Rule]) -> Update {
    let mut fixed_update = update.clone();
    let applicable_rules = get_applicable_rules(update, rules);
    while !is_right_update(&fixed_update, &applicable_rules) {
        for rule in &applicable_rules {
            let before = find_index(&fixed_update, &rule.before).unwrap();
            let after = find_index(&fixed_update, &rule.after).unwrap();
            if before > after {
                fixed_update.swap(before, after);
            }
        }
    }
    fixed_update
}

fn fix_order_updates(updates: &[Update], indexes: Vec<usize>, rules: Vec<Rule>) -> Vec<Update> {
    let mut fixed = Vec::new();
    for index in indexes {
        fixed.push(fix_single_update(&updates[index], &rules));
    }
    fixed
}

fn get_middle_page_numbers_sum(updates: Vec<Update>) -> u64 {
    updates.iter().map(get_middle_page_number).sum()
}

fn get_middle_page_number(update: &Update) -> u64 {
    update[update.len() / 2]
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let rules = get_rules(&lines);
    let updates = get_updates(&lines);

    let wrong_order_updates = get_wrong_order_updates(&updates, &rules);
    let fixed_updates = fix_order_updates(&updates, wrong_order_updates, rules);
    let middle_number_pages_sum = get_middle_page_numbers_sum(fixed_updates);

    println!("Middle number pages sum: {}", middle_number_pages_sum);
}
