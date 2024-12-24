use std::collections::{BinaryHeap, HashMap};
use shared::args::read_file_from_arg;
use shared::point::Point;
use shared::grid::Grid;


const EMPTY: char = '.';
const CORRUPTED: char = '#';

#[derive(Debug, PartialEq, Eq)]
struct Step {
    point: Point,
    previous_points: Vec<Point>,
}

impl Step {
    fn new(start: Point) -> Self {
        Self {
            point: start,
            previous_points: Vec::new(),
        }
    }

    fn next(&self, point: Point) -> Self {
        let mut previous_points = self.previous_points.clone();
        previous_points.push(self.point);

        Self {
            point,
            previous_points,
        }
    }

    fn cost(&self) -> usize {
        self.previous_points.len() + 1
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost().cmp(&self.cost())
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn parse_data(data: String) -> Vec<Point> {
    let mut points = Vec::new();

    for line in data.lines() {
        let (x, y) = line.split_once(",").expect("could not parse point");
        points.push(Point::new(
            x.parse().expect("could not parse x"),
            y.parse().expect("could not parse y")
        ))
    }

    points
}

fn populate_grid(grid: &Grid<char>, points: Vec<Point>) -> Grid<char> {
    let mut work = grid.clone();

    for point in points {
        work.set_point(&point, CORRUPTED);
    }

    work
}

fn shortest_path(grid: &Grid<char>, start: Point, end: Point) -> Option<Vec<Point>> {
    let mut queue: BinaryHeap<Step> = BinaryHeap::new();
    let mut visited: HashMap<Point, usize> = HashMap::new();

    queue.push(Step::new(start));

    while !queue.is_empty() {
        let step = queue.pop().unwrap();

        if step.point == end {
            return Some(step.previous_points);
        }

        if *visited.get(&step.point).unwrap_or(&usize::MAX) > step.cost() {
            visited.insert(step.point, step.cost());

            let contents = grid.get_point(&step.point);
            if contents == None || contents == Some(CORRUPTED) {
                continue;
            }

            for neighbor in step.point.direct_neighbors() {
                if !visited.contains_key(&neighbor) {
                    queue.push(step.next(neighbor));
                }
            }
        }
    }

    None
}

fn part2(grid: &Grid<char>, points: &Vec<Point>) -> Option<Point> {
    let mut work = grid.clone();

    let start = Point::new(0, 0);
    let end = Point::new((grid.width() - 1) as isize, (grid.height() - 1) as isize);

    for point in points {
        work.set_point(point, CORRUPTED);

        if shortest_path(&work, start, end) == None {
            return Some(*point);
        }
    }

    None
}


fn main() {
    let corruptions = parse_data(read_file_from_arg(1));

    let width = 71;
    let height = 71;
    let p1_points = 1024;

    let grid = Grid::<char>::empty_with_char(width, height, EMPTY);

    let p1_grid = populate_grid(&grid, corruptions[0..p1_points].to_vec());
    let path = shortest_path(
        &p1_grid,
        Point::new(0, 0),
        Point::new((width - 1) as isize, (height - 1) as isize)
    );
    println!("Part 1 Fewest Steps: {}", path.expect("did not find end").len());

    let blocker = part2(&grid, &corruptions);
    println!("Part 2 Blocker: {:?}", blocker);
}
