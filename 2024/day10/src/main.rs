use std::collections::HashSet;

use shared::args::get_reader_for_arg;
use shared::grid::Grid;
use shared::point::Point;


type Map = Grid<u8>;

#[derive(Debug)]
struct Trailhead {
    start: Point,
    score: usize,
}


fn summits_reached(map: &Map, point: &Point, level: u8, unique: bool) -> Vec<Point> {
    let mut summits = Vec::new();
    let next_level = level + 1;

    for next in point.direct_neighbors() {
        if !map.point_in_bounds(&next) { continue; }
        
        if let Some(next_value) = map.get_point(&next) {
            if next_value == next_level {
                if next_value == 9 {
                    summits.push(next);
                } else {
                    summits.extend(summits_reached(map, &next, next_value, unique));
                }
            }
        }
    }

    if unique {
        return Vec::from_iter(HashSet::<Point>::from_iter(summits));
    } else {
        return summits
    }
}

fn find_trailheads(map: &Map, score_unique: bool) -> Vec<Trailhead> {
    let mut trailheads = Vec::new();

    for i in map {
        if i.value == 0 {
            let trailhead = Trailhead {
                start: i.point,
                score: summits_reached(map, &i.point, 0, score_unique).len(),
            };
            trailheads.push(trailhead);
        }
    }

    trailheads
}


fn main() {
    let file = get_reader_for_arg(1);
    let map: Map = Grid::<u8>::digits_from_reader(file);

    let trailheads = find_trailheads(&map, true);
    let total: usize = trailheads.iter().map(|t| t.score).sum();
    println!("Part 1 Total Score: {}", total);

    let trailheads = find_trailheads(&map, false);
    let total: usize = trailheads.iter().map(|t| t.score).sum();
    println!("Part 2 Total Score: {}", total);
}
