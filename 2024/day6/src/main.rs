use std::collections::HashSet;
use std::fs;
use std::env;
use std::io;
use shared::grid::Grid;
use shared::grid_direction;
use shared::grid_direction::GridDirection;
use shared::point::Point;


#[derive(PartialEq, Clone, Copy)]
struct Position {
    point: Point,
    direction: GridDirection,
}

type Map = Grid<char>;
type Path = Vec<Position>;

const UP: char = '^';
const RIGHT: char = '>';
const DOWN: char = 'v';
const LEFT: char = '<';
const GUARD_MARKERS: [char; 4] = [UP, RIGHT, DOWN, LEFT];
const OBSTACLE: char = '#';
const EMPTY: char = '.';


fn find_guard(grid: &Map) -> Position {
    let guard = grid.into_iter().find(|i| GUARD_MARKERS.contains(&i.value)).unwrap();

    let direction;
    if guard.value == UP {
        direction = GridDirection::U;
    } else if guard.value == DOWN {
        direction = GridDirection::D;
    } else if guard.value == RIGHT {
        direction = GridDirection::R;
    } else if guard.value == LEFT {
        direction = GridDirection::L;
    } else {
        panic!("wat?")
    }

    Position {
        point: guard.point,
        direction,
    }
}

fn next_position(grid: &Map, current: Position) -> Position {
    let mut direction: GridDirection = current.direction;
    loop {
        let looking_at = current.point.translate(direction);
        if let Some(contents) = grid.get_point(&looking_at) {
            if contents == OBSTACLE {
                direction = grid_direction::turn_90_cw(direction);
            } else {
                break;
                }
        } else {
            break;
        }
    }

    let point = current.point.translate(direction);

    Position {
        point,
        direction,
    }
}

fn get_patrol_path(grid: &Map) -> (Path, bool) {
    let mut path: Path = Vec::new();

    let mut pos = find_guard(&grid);
    while grid.point_in_bounds(&pos.point) {
        path.push(pos);
        pos = next_position(grid, pos);

        if (path.len() > 1) && (path.contains(&pos)) {
            return (path, true);
        }
    }

    (path, false)
}

fn find_forced_loop_obstacles(grid: &Map) -> HashSet<Point> {
    let mut obstacles = HashSet::new();

    let mut working_grid = grid.clone();
    let (known_path, _) = get_patrol_path(grid);

    for i in 0..known_path.len() {
        if let Some(next) = known_path.get(i + 1) {
            if grid.get_point(&next.point).unwrap() != EMPTY { continue; }

            working_grid.set_point(&next.point, OBSTACLE);
            let (_, is_loop) = get_patrol_path(&working_grid);
            working_grid.set_point(&next.point, EMPTY);

            if is_loop {
                obstacles.insert(known_path[i + 1].point);
            }
        }
    }

    obstacles
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("must specify input file");
    let file = io::BufReader::new(fs::File::open(input_filename).expect("could not open file"));
    let grid = Grid::<char>::chars_from_reader(file);

    let (path, _) = get_patrol_path(&grid);
    let distinct_points: HashSet<Point> = HashSet::from_iter(path.iter().map(|p| p.point));
    println!("Part 1 Distinct Positions: {}", distinct_points.len());

    let obstacles = find_forced_loop_obstacles(&grid);
    println!("Part 2 Obstacles: {}", obstacles.len());
}
