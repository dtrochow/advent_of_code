use aoc_lib::{read_lines, Point};

/* Tiles per second in x and y direction */
type Velocity = Point;
type BathroomSize = Point;

/*
    x ->
  y  . . . .
  |  . . . .
  v  . . . .
     . . . .
*/
const X_SIZE: usize = 101;
const Y_SIZE: usize = 103;

#[derive(Debug)]
struct Robot {
    pos: Point,
    vel: Velocity,
}

fn get_robots(lines: Vec<String>) -> Vec<Robot> {
    lines
        .iter()
        .map(|r| {
            let pos_str = r
                .split_whitespace()
                .next()
                .unwrap()
                .split("p=")
                .last()
                .unwrap();
            let vel_str = r
                .split_whitespace()
                .last()
                .unwrap()
                .split("v=")
                .last()
                .unwrap();
            Robot {
                pos: Point {
                    x: pos_str.split(',').next().unwrap().parse().unwrap(),
                    y: pos_str.split(',').last().unwrap().parse().unwrap(),
                },
                vel: Velocity {
                    x: vel_str.split(',').next().unwrap().parse().unwrap(),
                    y: vel_str.split(',').last().unwrap().parse().unwrap(),
                },
            }
        })
        .collect()
}

fn update_robot_pos_after_seconds(robot: &mut Robot, seconds_num: usize, bathroom_size: &Point) {
    let points_to_move_x = (seconds_num as i64 * robot.vel.x) % bathroom_size.x;

    let (pos_x_after, pos_y_after);
    if robot.pos.x + points_to_move_x > (bathroom_size.x - 1) {
        /* Moving right + teleport */
        pos_x_after = points_to_move_x.abs() - (bathroom_size.x - robot.pos.x);
    } else if robot.pos.x + points_to_move_x < 0 {
        /* Moving left + teleport */
        pos_x_after = bathroom_size.x - (points_to_move_x.abs() - robot.pos.x);
    } else {
        /* Position fits in the bathroom */
        pos_x_after = robot.pos.x + points_to_move_x;
    }

    let points_to_move_y = (seconds_num as i64 * robot.vel.y) % bathroom_size.y;
    if robot.pos.y + points_to_move_y > (bathroom_size.y - 1) {
        /* Moving down + teleport */
        pos_y_after = points_to_move_y.abs() - (bathroom_size.y - robot.pos.y);
    } else if robot.pos.y + points_to_move_y < 0 {
        /* Moving up + teleport */
        pos_y_after = bathroom_size.y - (points_to_move_y.abs() - robot.pos.y);
    } else {
        /* Position fits in the bathroom */
        pos_y_after = robot.pos.y + points_to_move_y;
    }

    robot.pos = Point {
        x: pos_x_after,
        y: pos_y_after,
    };
}

fn print_robots_arrangement(robots: &Vec<Robot>) {
    const ROBOT_MARK: char = 'o';

    let mut bathroom_floor: Vec<Vec<usize>> = vec![vec![0; X_SIZE]; Y_SIZE];
    for robot in robots {
        let pos = robot.pos;
        bathroom_floor[pos.y as usize][pos.x as usize] += 1;
    }
    println!("{}", '_'.to_string().repeat(X_SIZE + 2));
    for row in bathroom_floor {
        print!("|");
        for robot in row {
            if robot > 0 {
                print!("{}", ROBOT_MARK);
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    println!("{}", '^'.to_string().repeat(X_SIZE + 2));
}

fn move_robots(robots: &mut Vec<Robot>, bathroom_size: &Point, seconds: usize) {
    for robot in robots {
        update_robot_pos_after_seconds(robot, seconds, bathroom_size);
    }
}

fn calculate_standard_deviation(robots: &Vec<Robot>) -> f64 {
    let mean: f64 =
        (robots.iter().map(|r| r.pos.x + r.pos.y).sum::<i64>() / robots.len() as i64) as f64;
    let mut sum_squares: f64 = 0.0;
    for robot in robots {
        sum_squares += (robot.pos.x as f64 + robot.pos.y as f64 - mean).powi(2);
    }
    (sum_squares / robots.len() as f64).sqrt()
}

fn is_tree_picture(robots: &Vec<Robot>) -> bool {
    let sd = calculate_standard_deviation(robots);
    if sd < 27.0 {
        println!("SD: {}", sd);
        return true;
    }
    false
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let mut robots = get_robots(lines);

    let bathroom_size = BathroomSize {
        x: X_SIZE as i64,
        y: Y_SIZE as i64,
    };

    let mut seconds_cnt = 0;
    while !is_tree_picture(&robots) {
        move_robots(&mut robots, &bathroom_size, 1);
        seconds_cnt += 1;
    }
    print_robots_arrangement(&robots);
    println!("Seconds count: {}", seconds_cnt);
}
