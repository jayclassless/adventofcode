use std::collections::{BinaryHeap, HashMap, HashSet};
use shared::args::get_reader_for_arg;
use shared::grid::Grid;
use shared::grid_direction;
use shared::grid_direction::GridDirection;
use shared::point::Point;


type Maze = Grid<char>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Step {
    point: Point,
    previous_points: Vec<Point>,
    direction: GridDirection,
    cost: usize,
}

impl Step {
    fn new(point: Point, direction: GridDirection, cost: usize) -> Self {
        Self {
            point,
            previous_points: Vec::new(),
            direction,
            cost,
        }
    }

    fn next(&self) -> Self {
        let mut previous_points = self.previous_points.clone();
        previous_points.push(self.point);

        Self {
            point: self.point.translate(self.direction),
            previous_points,
            direction: self.direction,
            cost: self.cost + 1,
        }
    }

    fn turn(&self, direction: GridDirection) -> Self {
        let mut previous_points = self.previous_points.clone();
        previous_points.push(self.point);

        Self {
            point: self.point,
            previous_points,
            direction: direction,
            cost: self.cost + 1000,
        }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';


fn endpoints(maze: &Maze) -> (Point, Point) {
    (
        maze.into_iter().find(|i| i.value == START).expect("could not find start").point,
        maze.into_iter().find(|i| i.value == END).expect("could not find end").point,
    )
}

fn lowest_cost(maze: &Maze) -> usize {
    let mut queue: BinaryHeap<Step> = BinaryHeap::new();
    let mut visited: HashMap<(Point, GridDirection), usize> = HashMap::new();

    let (start, end) = endpoints(&maze);
    queue.push(Step::new(start, GridDirection::R, 0));

    while !queue.is_empty() {
        let step = queue.pop().unwrap();

        if step.point == end {
            return step.cost;
        }

        let visited_key = (step.point, step.direction);
        if *visited.get(&visited_key).unwrap_or(&usize::MAX) > step.cost {
            visited.insert(visited_key, step.cost);

            let next = step.point.translate(step.direction);
            if let Some(contents) = maze.get_point(&next) {
                if contents != WALL {
                    queue.push(step.next());
                }
            }

            queue.push(step.turn(grid_direction::turn_90_cw(step.direction)));
            queue.push(step.turn(grid_direction::turn_90_ccw(step.direction)));
        }

    }

    panic!("never found end!");
}

fn lowest_cost_points(maze: &Maze) -> Vec<Point> {
    let mut queue: BinaryHeap<Step> = BinaryHeap::new();
    let mut visited: HashMap<(Point, GridDirection), usize> = HashMap::new();
    let mut all_points: HashSet<Point> = HashSet::new();
    let mut found_end = false;
    let mut target_cost: usize = 0;

    let (start, end) = endpoints(&maze);
    queue.push(Step::new(start, GridDirection::R, 0));

    while !queue.is_empty() {
        let step = queue.pop().unwrap();

        if found_end && step.cost > target_cost {
            return Vec::from_iter(all_points);
        }

        if step.point == end {
            found_end = true;
            target_cost = step.cost;
            all_points.insert(step.point);
            all_points.extend(step.previous_points.iter());
        }

        let visited_key = (step.point, step.direction);
        if *visited.get(&visited_key).unwrap_or(&usize::MAX) >= step.cost {
            visited.insert(visited_key, step.cost);

            let next = step.point.translate(step.direction);
            if let Some(contents) = maze.get_point(&next) {
                if contents != WALL {
                    queue.push(step.next());
                }
            }

            queue.push(step.turn(grid_direction::turn_90_cw(step.direction)));
            queue.push(step.turn(grid_direction::turn_90_ccw(step.direction)));
        }

    }

    Vec::from_iter(all_points)
}


fn main() {
    let file = get_reader_for_arg(1);
    let maze: Maze = Grid::<char>::chars_from_reader(file);

    let cost = lowest_cost(&maze);
    println!("Part 1 Cost: {}", cost);
    let points = lowest_cost_points(&maze);
    println!("Part 2 Sots: {}", points.len());
}
