use aoc_lib::read_lines;

type DiskMap = Vec<u32>;
type DiskBlocks = Vec<Option<u64>>;

fn get_disk_map(lines: Vec<String>) -> DiskMap {
    lines
        .first()
        .unwrap()
        .chars()
        .map(|i| i.to_digit(10).unwrap())
        .collect()
}

fn is_free_space(index: usize) -> bool {
    index % 2 != 0
}

fn get_id_number(index: usize) -> u64 {
    (index / 2).try_into().unwrap()
}

fn get_disk_blocks(disk_map: DiskMap) -> DiskBlocks {
    disk_map
        .iter()
        .enumerate()
        .flat_map(|(index, digit)| {
            (0..*digit).map(move |_| {
                if is_free_space(index) {
                    None
                } else {
                    Some(get_id_number(index))
                }
            })
        })
        .collect()
}

fn find_last_file_index(disk_blocks: &DiskBlocks) -> Option<usize> {
    for (i, digit) in disk_blocks.iter().rev().enumerate() {
        if digit.is_some() {
            return Some(disk_blocks.len() - 1 - i);
        }
    }
    None
}

fn defragment_disk(disk_blocks: &mut DiskBlocks) {
    for i in 0..disk_blocks.len() {
        if disk_blocks[i].is_none() {
            let last_file_index = find_last_file_index(disk_blocks).unwrap();
            disk_blocks.swap(i, last_file_index);
        }
    }
    let first_empty_index =
        find_last_file_index(&disk_blocks.split_last().unwrap().1.to_vec()).unwrap() + 1;
    let last_index = disk_blocks.len() - 1;
    disk_blocks.swap(first_empty_index, last_index);
}

fn calculate_checksum(disk_blocks: DiskBlocks) -> u64 {
    disk_blocks
        .iter()
        .enumerate()
        .map(|(i, id_num)| id_num.unwrap_or(0) * i as u64)
        .sum()
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let disk_map = get_disk_map(lines);
    let mut disk_blocks = get_disk_blocks(disk_map);

    defragment_disk(&mut disk_blocks);

    let checksum = calculate_checksum(disk_blocks);
    println!("Checksum: {}", checksum);
}
