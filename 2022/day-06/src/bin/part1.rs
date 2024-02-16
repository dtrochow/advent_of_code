use std::fs::read_to_string;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let line: String = read_lines("./src/bin/input1.txt").first().unwrap().to_string();
    let marker_index = find_marker_index(line);
    println!("Marker index: {}", marker_index);

    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn find_marker_index(line: String) -> u32 {
    let distinct_characters_count: usize = 4;
    let mut marker: VecDeque<char> = line[0..distinct_characters_count].chars().collect();
    let mut marker_index: u32 = std::u32::MAX;
    for (index, ch) in line.chars().enumerate().skip(4) {
        if has_unique_elements(marker.clone()) {
            marker_index = index as u32;
            break;
        }
        marker.pop_front();
        marker.push_back(ch); 
    }
    println!("Marker values: {marker:?}");
    marker_index
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
