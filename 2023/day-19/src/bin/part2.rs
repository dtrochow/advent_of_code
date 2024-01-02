use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

type PartRating = HashMap<char, u32>;

struct Condition {
    part: char, // x, m, a, s
    value: u32,
    is_part_greater_sign: bool, // < or >
    condition_result: String,   // A, R or workflow_name
}

type Workflows = HashMap<String, Vec<Condition>>;

enum Result {
    Accepted,
    Rejected,
    NextWorkflow(String),
    NextCondition,
}

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let mut workflows: Workflows = HashMap::new();
    for workflow in &lines {
        if workflow.is_empty() {
            break;
        }
        let (name, conditions) = parse_workflow(workflow);
        workflows.entry(name).or_insert(conditions);
    }

    let mut part_ratings: Vec<PartRating> = Vec::new();
    for rating in lines.iter().rev() {
        if rating.is_empty() {
            break;
        }
        part_ratings.push(parse_part_rating(rating));
    }

    let mut sum: u32 = 0;
    for part in part_ratings {
        if let Some(rating) = check_part(&part, &workflows) {
            sum += rating;
        }
    }

    println!("Sum: {}", sum);

    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

fn check_part(part: &PartRating, workflows: &Workflows) -> Option<u32> {
    let mut current_workflow: String = "in".to_string();
    loop {
        for condition in workflows.get(&current_workflow).unwrap() {
            match check_condition(condition, part) {
                Result::Accepted => {
                    return Some(sum_part_ratings(part));
                }
                Result::Rejected => {
                    return None;
                }
                Result::NextWorkflow(workflow) => {
                    current_workflow = workflow;
                    break;
                }
                Result::NextCondition => (),
            }
        }
    }
}

fn sum_part_ratings(part: &PartRating) -> u32 {
    part.values().sum()
}

fn check_condition(condition: &Condition, part: &PartRating) -> Result {
    if condition.value == 0 {
        return check_if_accepted(condition);
    }

    let part_value = *part.get(&condition.part).unwrap();
    if condition.is_part_greater_sign {
        if part_value > condition.value {
            check_if_accepted(condition)
        } else {
            Result::NextCondition
        }
    } else if part_value < condition.value {
        check_if_accepted(condition)
    } else {
        Result::NextCondition
    }
}

fn check_if_accepted(condition: &Condition) -> Result {
    match condition.condition_result.as_str() {
        "A" => Result::Accepted,
        "R" => Result::Rejected,
        _ => Result::NextWorkflow(condition.condition_result.clone()),
    }
}

fn parse_part_rating(rating_string: &String) -> PartRating {
    let mut part_rating: PartRating = HashMap::new();
    let parts = &rating_string[1..rating_string.len() - 1];
    for part in parts.split(',').collect::<Vec<&str>>() {
        part_rating
            .entry(part.split('=').next().unwrap().parse::<char>().unwrap())
            .or_insert(part.split('=').last().unwrap().parse::<u32>().unwrap());
    }
    part_rating
}

fn parse_workflow(workflow_string: &String) -> (String, Vec<Condition>) {
    let name: String = workflow_string
        .clone()
        .split('{')
        .next()
        .unwrap()
        .to_string();
    let mut conditions: Vec<Condition> = Vec::new();

    let conditions_string = &workflow_string[name.len() + 1..workflow_string.len() - 1];
    for cond in conditions_string.split(',').collect::<Vec<&str>>() {
        conditions.push(parse_condition(cond));
    }

    (name, conditions)
}

fn parse_condition(condition_str: &str) -> Condition {
    if !condition_str.contains(':') {
        Condition {
            part: ' ',
            value: 0,
            is_part_greater_sign: false,
            condition_result: condition_str.to_string(),
        }
    } else {
        Condition {
            part: condition_str.chars().next().unwrap(),
            value: condition_str.split(':').next().unwrap()[2..]
                .to_string()
                .parse::<u32>()
                .unwrap(),
            is_part_greater_sign: condition_str.chars().nth(1).unwrap() == '>',
            condition_result: condition_str.split(':').last().unwrap().to_string(),
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
