use std::env;
use std::fs;
use shared::grid::Grid;
use shared::grid_direction;
use shared::grid_direction::GridDirection;
use shared::point::Point;


type Map = Grid<char>;
type Movements = Vec<GridDirection>;

const EMPTY: char = '.';
const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';


fn parse_file(path: &str) -> (Map, Point, Movements) {
    let data = fs::read_to_string(path).expect("could not read file");

    let (map_data, move_data) = data.split_once("\n\n").expect("parse error");

    let map = Grid::<char>::chars_from_string(map_data);

    let start = map
        .into_iter()
        .find(|p| p.value == ROBOT).expect("could not find robot")
        .point
    ;

    let movements: Movements = move_data
        .chars()
        .map(|c| grid_direction::from_char(c))
        .filter(|d| !d.is_none())
        .map(|d| d.unwrap())
        .collect()
    ;

    (map, start, movements)
}

fn move_object(map: &mut Map, position: &Point, direction: GridDirection) -> Point {
    let object = map.get_point(position).unwrap();
    let next = position.translate(direction);
    let contents = map.get_point(&next).unwrap();

    if contents == EMPTY {
        map.set_point(&next, object);
        map.set_point(position, EMPTY);
        return next;
    }

    if contents == WALL {
        return *position;
    }

    if contents == BOX {
        let moved = move_object(map, &next, direction);
        if moved != next {
            map.set_point(&next, object);
            map.set_point(position, EMPTY);
            return next;
        }
        return *position;
    }

    panic!("huh?")
}

fn move_object_multi(map: &mut Map, position: &Point, movements: &Movements) -> Point {
    let mut current = position.clone();

    for direction in movements {
        current = move_object(map, &current, *direction);
    }

    current
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("must specify input file");

    let (mut map, position, movements) = parse_file(input_filename);
    move_object_multi(&mut map, &position, &movements);

    let total: isize = map.into_iter()
        .filter(|i| i.value == BOX)
        .map(|i| (i.point.y * 100) + i.point.x)
        .sum()
    ;
    println!("Part 1 Total: {}", total);
}
