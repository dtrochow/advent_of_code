#[macro_use]
extern crate maplit;

use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

// {node_point: {node_point1: distance, node_point2: distance}}
type Graph = HashMap<Point, HashMap<Point, u32>>;

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = read_lines("./src/bin/input1.txt");

    let mut grid: Grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }

    // find start
    let start: Point = find_start_point(&grid);

    // find end
    let end: Point = find_end_point(&grid);

    // find all nodes
    let mut nodes: Vec<Point> = find_all_nodes(&grid);
    nodes.push(start);
    nodes.push(end);

    // build graph
    let graph: Graph = build_graph(&grid, &nodes);

    // find the longest road
    let mut seen: HashSet<Point> = HashSet::new();
    let longest_road = dfs(&start, &end, &graph, &mut seen);
    println!("Longest road: {}", longest_road);

    println!(
        "Elapsed time: {}s {}ms",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}

// Depth-first search
fn dfs(point: &Point, end: &Point, graph: &Graph, seen: &mut HashSet<Point>) -> i32 {
    if point == end {
        return 0;
    }

    let mut m = 0;
    seen.insert(*point);
    for nx in graph.get(point).unwrap().keys() {
        m = cmp::max(m, dfs(nx, end, graph, seen) + graph[point][nx] as i32);
    }
    seen.remove(point);

    m
}

fn build_graph(grid: &Grid, nodes: &Vec<Point>) -> Graph {
    let mut graph: Graph = HashMap::new();

    for node in nodes {
        graph.insert(*node, HashMap::new());
        find_node_connections(grid, node, nodes, &mut graph);
    }
    graph
}

fn find_node_connections(grid: &Grid, node: &Point, nodes: &[Point], graph: &mut Graph) {
    let mut stack: Vec<(u32, Point)> = vec![(0, *node)];
    let mut seen: HashSet<Point> = HashSet::from([*node]);

    let directions: HashMap<char, Vec<(i32, i32)>> = hashmap! {
        '>' => vec![(1, 0)],
        '<' => vec![(-1, 0)],
        '^' => vec![(0, -1)],
        'v' => vec![(0, 1)],
        '.' => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
    };

    while !stack.is_empty() {
        let (n, point) = stack.pop().unwrap();

        if n != 0 && nodes.contains(&point) {
            graph.get_mut(node).unwrap().entry(point).or_insert(n);
            continue;
        }

        for direction in directions.get(&grid[point.y][point.x]).unwrap() {
            let ny: i32 = point.y as i32 + direction.1;
            let nx: i32 = point.x as i32 + direction.0;
            if nx >= 0
                && nx < grid.first().unwrap().len() as i32
                && ny >= 0
                && ny < grid.len() as i32
            {
                let new_point: Point = Point {
                    x: nx as usize,
                    y: ny as usize,
                };
                if !seen.contains(&new_point) && grid[ny as usize][nx as usize] != '#' {
                    stack.push((n + 1, new_point));
                    seen.insert(new_point);
                }
            }
        }
    }
}

fn find_all_nodes(grid: &Grid) -> Vec<Point> {
    let mut nodes: Vec<Point> = Vec::new();
    for (ri, row) in grid.iter().enumerate() {
        for (ci, ch) in row.iter().enumerate() {
            if *ch == '#' {
                continue;
            }

            let mut neighbors_count = 0;
            let x_pos = ci as i32;
            let y_pos = ri as i32;
            for point in &[
                (x_pos - 1, y_pos),
                (x_pos + 1, y_pos),
                (x_pos, y_pos - 1),
                (x_pos, y_pos + 1),
            ] {
                if point.0 >= 0
                    && point.0 < row.len() as i32
                    && point.1 >= 0
                    && point.1 < grid.len() as i32
                    && grid[point.1 as usize][point.0 as usize] != '#'
                {
                    neighbors_count += 1
                }
            }
            if neighbors_count >= 3 {
                nodes.push(Point { x: ci, y: ri });
            }
        }
    }
    nodes
}

fn find_start_point(grid: &Grid) -> Point {
    let first_row: String = grid.first().unwrap().iter().collect();
    Point {
        x: first_row.find('.').unwrap(),
        y: 0,
    }
}

fn find_end_point(grid: &Grid) -> Point {
    let last_row: String = grid.last().unwrap().iter().collect();
    Point {
        x: last_row.find('.').unwrap(),
        y: (grid.len() - 1),
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
