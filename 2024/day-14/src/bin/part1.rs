use aoc_lib::{get_quadrant_borders, read_lines, Point, QuadrantId};
use strum::IntoEnumIterator;

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
const SECONDS_NUMBER: usize = 100;

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

fn get_robot_pos_after_seconds(robot: &Robot, seconds_num: usize, bathroom_size: &Point) -> Point {
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

    Point {
        x: pos_x_after,
        y: pos_y_after,
    }
}

fn count_robots_in_quadrant(
    robots_pos: &[Point],
    bathroom_size: &Point,
    q_id: QuadrantId,
) -> usize {
    let borders = get_quadrant_borders(q_id, bathroom_size);
    robots_pos
        .iter()
        .filter(|pos| borders.is_within_borders(pos))
        .count()
}

fn print_robots_arrangement(robots_pos: &Vec<Point>) {
    const ROBOT_MARK: char = 'O';

    let mut bathroom_floor: Vec<Vec<usize>> = vec![vec![0; X_SIZE]; Y_SIZE];
    for pos in robots_pos {
        bathroom_floor[pos.y as usize][pos.x as usize] += 1;
    }
    for row in bathroom_floor {
        for robot in row {
            if robot > 0 {
                print!("{}", ROBOT_MARK);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let robots = get_robots(lines);

    let bathroom_size = BathroomSize {
        x: X_SIZE as i64,
        y: Y_SIZE as i64,
    };
    let mut robot_positions_after: Vec<Point> = Vec::new();
    for robot in robots {
        robot_positions_after.push(get_robot_pos_after_seconds(
            &robot,
            SECONDS_NUMBER,
            &bathroom_size,
        ));
    }

    print_robots_arrangement(&robot_positions_after);

    let mut quadrant_robots_num: Vec<usize> = Vec::new();
    for q_id in QuadrantId::iter() {
        let robots_num =
            count_robots_in_quadrant(&robot_positions_after, &bathroom_size, q_id.clone());
        quadrant_robots_num.push(robots_num);
    }

    let safety_factor: usize = quadrant_robots_num.iter().product();
    println!("Safety factor: {}", safety_factor);
}
