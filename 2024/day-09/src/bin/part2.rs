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

fn get_disk_blocks(disk_map: &DiskMap) -> DiskBlocks {
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

/* Returns (start_index, size) */
fn get_file(disk_blocks: &DiskBlocks, file_id: u64) -> Option<(usize, usize)> {
    let mut size = 0;
    for (index, id) in disk_blocks.iter().enumerate().rev() {
        if id.is_some() {
            if size != 0 && (id.unwrap() != file_id || index == 0) {
                if index == 0 {
                    size += 1
                }
                return Some((index + 1, size));
            }
            if id.unwrap() == file_id {
                size += 1;
            }
        } else if size != 0 {
            return Some((index + 1, size));
        }
    }
    None
}

/* Returns index to the beginning of requested empty space */
fn find_empty_space(disk_blocks: &DiskBlocks, size: &usize) -> Option<usize> {
    let mut empty_size = 0;
    for (index, block) in disk_blocks.iter().enumerate() {
        if block.is_none() {
            empty_size += 1;
        } else {
            empty_size = 0;
        }
        if empty_size == *size {
            return Some(index - size + 1);
        }
    }
    None
}

fn move_file(disk_blocks: &mut DiskBlocks, file_i: usize, empty_i: usize, size: usize) {
    for i in 0..size {
        disk_blocks.swap(file_i + i, empty_i + i);
    }
}

fn defragment_disk(disk_blocks: &mut DiskBlocks, files_cnt: usize) {
    for file_id in (0..files_cnt).rev() {
        let (file_index, file_size) = get_file(disk_blocks, file_id as u64).unwrap();
        let empty_space = find_empty_space(disk_blocks, &file_size);
        if let Some(empty_index) = empty_space {
            if empty_index < file_index {
                move_file(disk_blocks, file_index, empty_index, file_size);
            }
        }
    }
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
    let mut disk_blocks = get_disk_blocks(&disk_map);

    let files_cnt = disk_map.len() / 2 + 1;
    defragment_disk(&mut disk_blocks, files_cnt);
    let checksum = calculate_checksum(disk_blocks);
    println!("Checksum: {}", checksum);
}
