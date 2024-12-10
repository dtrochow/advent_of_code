use aoc_lib::{find_all_value_positions, is_in_2d_map, read_lines, Point, DIRECTIONS};

type HeightMap = Vec<Vec<i8>>;

fn get_height_map(lines: Vec<String>) -> HeightMap {
    lines
        .iter()
        .map(|row| row.chars().map(|h| h.to_digit(10).unwrap() as i8).collect())
        .collect()
}

fn find_all_trail_heads(height_map: &HeightMap) -> Vec<Point> {
    find_all_value_positions(height_map, 0)
}

fn find_all_hiking_trails_from_head(height_map: &HeightMap, trail_head: Point) -> usize {
    let mut trails: Vec<Point> = vec![trail_head];
    let mut hiking_trails = 0;

    while let Some(trail) = trails.pop() {
        let current_height = height_map[trail.x as usize][trail.y as usize];
        for direction in DIRECTIONS {
            let next_position = trail.clone() + direction.clone();
            if !is_in_2d_map(height_map, &next_position) {
                continue;
            }
            let next_height = height_map[next_position.x as usize][next_position.y as usize];
            if next_height == 9 && next_height - current_height == 1 {
                hiking_trails += 1;
                continue;
            }
            if next_height - current_height == 1 {
                trails.push(next_position);
            }
        }
    }
    hiking_trails
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let height_map = get_height_map(lines);
    let trail_heads = find_all_trail_heads(&height_map);

    let mut hiking_trails_cnt = 0;
    for trail_head in trail_heads {
        let score = find_all_hiking_trails_from_head(&height_map, trail_head.clone());
        hiking_trails_cnt += score;
    }

    println!("Hiking trails count: {}", hiking_trails_cnt);
}
