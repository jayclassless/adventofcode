use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use itertools::Itertools;
use shared::grid::Grid;
use shared::point::Point;


type Map = Grid<char>;

const EMPTY: char = '.';


fn group_antennas(map: &Map) -> HashMap<char, Vec<Point>> {
    let mut groups = HashMap::new();

    for i in map {
        if i.value == EMPTY { continue; }

        groups
            .entry(i.value)
            .or_insert(Vec::new())
            .push(i.point);
    }

    groups
}

fn antinodes_for_antennas(map: &Map, first: &Point, second: &Point) -> Vec<Point> {
    let mut nodes = Vec::new();

    let dx = second.x - first.x;
    let dy = second.y - first.y;

    let mut next = *first;
    loop {
        next = Point {
            x: next.x - dx,
            y: next.y - dy,
        };

        if map.point_in_bounds(&next) {
            nodes.push(next);
        } else {
            break;
        }
    }

    nodes
}

fn find_antinodes_part1(map: &Map) -> Vec<Point> {
    let mut antinodes = Vec::new();

    for (_, annennas) in group_antennas(map) {
        for pair in annennas.iter().permutations(2) {
            let nodes = antinodes_for_antennas(map, pair[0], pair[1]);
            if nodes.len() > 0 {
                antinodes.push(nodes[0])
            }
        }
    }

    antinodes
}

fn find_antinodes_part2(map: &Map) -> Vec<Point> {
    let mut antinodes = Vec::new();

    for (_, annennas) in group_antennas(map) {
        for pair in annennas.iter().permutations(2) {
            let nodes = antinodes_for_antennas(map, pair[0], pair[1]);
            antinodes.extend(nodes);
        }

        antinodes.extend(annennas);
    }

    antinodes
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("must specify input file");
    let file = io::BufReader::new(fs::File::open(input_filename).expect("could not open file"));
    let map: Map = Grid::<char>::chars_from_reader(file);

    let antinodes = find_antinodes_part1(&map);
    println!("Part 1 Locations: {}", antinodes.iter().unique().count());

    let antinodes = find_antinodes_part2(&map);
    println!("Part 2 Locations: {}", antinodes.iter().unique().count());
}
