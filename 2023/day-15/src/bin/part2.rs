use array_init::array_init;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(PartialEq, Debug)]
enum LenseOp {
    Put,
    Remove,
}

#[derive(Debug)]
struct StepDesc {
    label_hash: u8,
    label: String,
    op: LenseOp,
    value: u32,
}

#[derive(Default, Debug)]
struct Lense {
    label: String,
    value: u32,
}

const NUMBER_OF_BOXES: usize = 256;

type Box = Vec<Lense>;
type Boxes = [Box; NUMBER_OF_BOXES];

fn main() {
    let now = Instant::now();

    let line: String = read_lines("./src/bin/input1.txt")
        .first()
        .unwrap()
        .to_string();
    let steps: Vec<String> = get_all_steps(line);

    let mut steps_descriptors: Vec<StepDesc> = Vec::new();
    for step in steps {
        let label: String;
        let mut operation: LenseOp = LenseOp::Remove;
        let mut value: u32 = 0;
        if step.contains('-') {
            label = step.split('-').next().unwrap().to_string();
        } else {
            label = step.split('=').next().unwrap().to_string();
            value = step.split('=').last().unwrap().parse().unwrap();
            operation = LenseOp::Put;
        }
        let hash: u8 = calculate_hash_value(label.clone());
        steps_descriptors.push(StepDesc {
            label_hash: hash,
            label,
            op: operation,
            value,
        });
    }

    let mut boxes: Boxes = array_init(|_| Default::default());
    for step_desc in steps_descriptors {
        if step_desc.op == LenseOp::Put {
            put_lense_into_box(&mut boxes, &step_desc);
        } else if step_desc.op == LenseOp::Remove {
            remove_lense_from_box(&mut boxes, &step_desc);
        } else {
            println!("ERROR");
        }
    }

    let focusing_power = calculate_focusing_power(&boxes);
    println!("Final focusing power: {}", focusing_power);

    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

fn calculate_focusing_power(boxes: &Boxes) -> u32 {
    let mut focusing_power: u32 = 0;
    for (box_index, single_box) in boxes.iter().enumerate() {
        for (lense_index, lense) in single_box.iter().enumerate() {
            focusing_power += (box_index as u32 + 1) * (lense_index as u32 + 1) * lense.value;
        }
    }
    focusing_power
}

fn remove_lense_from_box(boxes: &mut Boxes, step_desc: &StepDesc) {
    if let Some(box_index) =
        is_label_in_box(&boxes[step_desc.label_hash as usize], &step_desc.label)
    {
        boxes[step_desc.label_hash as usize].remove(box_index);
    }
}

fn put_lense_into_box(boxes: &mut [Box], step_desc: &StepDesc) {
    if let Some(box_index) =
        is_label_in_box(&boxes[step_desc.label_hash as usize], &step_desc.label)
    {
        boxes[step_desc.label_hash as usize][box_index].value = step_desc.value;
    } else {
        boxes[step_desc.label_hash as usize].push(Lense {
            label: step_desc.label.clone(),
            value: step_desc.value,
        });
    }
}

fn calculate_hash_value(step: String) -> u8 {
    let mut hash: u32 = 0;
    for ch in step.chars() {
        let ascii_value: u8 = ch as u8;
        hash = ((hash + ascii_value as u32) * 17) % 256;
    }
    hash as u8
}

fn is_label_in_box(single_box: &Box, label: &String) -> Option<usize> {
    for (index, lense) in single_box.iter().enumerate() {
        if lense.label == *label {
            return Some(index);
        }
    }
    None
}

fn get_all_steps(line: String) -> Vec<String> {
    line.split(',')
        .map(|s| s.parse().expect("Parsing error"))
        .collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
