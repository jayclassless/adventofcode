use std::env;
use std::fs;
use std::usize;
use shared::point::Point;


#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Point,
}

struct Area {
    width: isize,
    height: isize,
}


fn parse_file(path: &str) -> Vec<Robot> {
    let mut robots = Vec::new();

    for line in fs::read_to_string(path).expect("could not read file").lines() {
        let (p, v) = line.split_once(" ").expect("could not parse line");

        let (x, y) = p[2..].split_once(",").expect("could not parse p");
        let position = Point::new(x.parse().expect("bad px"), y.parse().expect("bad py"));

        let (x, y) = v[2..].split_once(",").expect("could not parse v");
        let velocity = Point::new(x.parse().expect("bad vx"), y.parse().expect("bad vy"));

        robots.push(Robot { position, velocity });
    }

    robots
}

fn move_robot(robot: &Robot, area: &Area) -> Robot {
    let mut new_position = robot.position + robot.velocity;

    if new_position.x < 0 {
        new_position.x += area.width;
    } else if new_position.x > (area.width - 1) {
        new_position.x -= area.width;
    }

    if new_position.y < 0 {
        new_position.y += area.height;
    } else if new_position.y > (area.height - 1) {
        new_position.y -= area.height;
    }

    Robot {
        position: new_position,
        velocity: robot.velocity,
    }
}

fn move_robot_n(robot: &Robot, area: &Area, n: usize) -> Robot {
    let mut updated = robot.clone();

    for _ in 0..n {
        updated = move_robot(&updated, area);
    }

    updated
}

fn count_quadrants(robots: &Vec<Robot>, area: &Area) -> [usize; 4] {
    let mut counts = [0, 0, 0, 0];

    let mid_x = area.width / 2;
    let mid_y = area.height / 2;

    for robot in robots {
        if robot.position.x < mid_x && robot.position.y < mid_y {
            counts[0] += 1;
        } else if robot.position.x > mid_x && robot.position.y < mid_y {
            counts[1] += 1;
        } else if robot.position.x < mid_x && robot.position.y > mid_y {
            counts[2] += 1;
        } else if robot.position.x > mid_x && robot.position.y > mid_y {
            counts[3] += 1;
        }
    } 

    counts
}

fn safety_factor(robots: &Vec<Robot>, area: &Area) -> usize {
    count_quadrants(robots, area).iter().map(|q| *q).product()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("must specify input file");

    let robots = parse_file(input_filename);
    let area = Area { width: 101, height: 103 };

    let moved_robots: Vec<Robot> = robots.iter().map(|r| move_robot_n(r, &area, 100)).collect();
    println!("Part 1 Safety Factor: {}", safety_factor(&moved_robots, &area));

    let mut min_safety: usize = usize::MAX;
    let mut min_safety_idx: usize = 0;
    let mut next = robots.clone();
    for i in 0..(area.width * area.height) {
        next = next.iter().map(|r| move_robot(r, &area)).collect();
        let safety = safety_factor(&next, &area);
        if safety < min_safety {
            min_safety = safety;
            min_safety_idx = i as usize;
        }
    }
    println!("Part 2 Seconds: {}", min_safety_idx + 1);
}
