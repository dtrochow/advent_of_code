use std::fs::read_to_string;
use std::time::Instant;

struct RowDesc {
    row: String,
    groups: Vec<u32>
}

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let row_descriptors: Vec<RowDesc> = get_row_descriptors(lines);
    // for row_desc in row_descriptors {
    //     println!("Row: {} Groups: {:?}", row_desc.row, row_desc.groups);
    // }

    println!("Elapsed time: {}s {}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}

fn get_row_descriptors(lines: Vec<String>) -> Vec<RowDesc> {
    let mut row_descriptors: Vec<RowDesc> = Vec::new();
    for line in lines {
        let row_str: String = line.split_whitespace().next().unwrap().to_string();
        let groups: Vec<u32> = line.split_whitespace().last().unwrap().split(',')
                                                      .map(|s| s.parse().expect("Parsing error"))
                                                      .collect();
        row_descriptors.push(RowDesc{row: row_str, groups: groups});
    }
    row_descriptors
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
