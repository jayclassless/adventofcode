use std::fs;
use std::env;
use std::io;
use shared::grid::Grid;
use shared::point::Point;
use shared::grid_direction::{GridDirection, ALL_GRID_DIRECTIONS};


fn search_word(grid: &Grid<char>, point: &Point, direction: GridDirection, word: &[char]) -> usize {
    if word.len() == 0 {
        return 1;
    }

    let cur = grid.get_point(point);
    if cur.is_none() || cur.unwrap() != word[0] {
        return 0;
    }

    if word.len() == 1 {
        return 1;
    }

    let next = point.translate(direction);

    if grid.point_in_bounds(&next) {
        return search_word(grid, &next, direction, &word[1..]);
    }

    return 0;
}

fn search_board(grid: &Grid<char>, word: &[char]) -> usize {
    let mut hits = 0;

    for i in grid {
        if i.value != word[0] { continue; }

        for direction in ALL_GRID_DIRECTIONS {
            hits += search_word(grid, &i.point, direction, &word);
        }
    }
    
    hits
}

const CROSS_DIRECTIONS: [(GridDirection, GridDirection); 4] = [
    (GridDirection::UR, GridDirection::DL),
    (GridDirection::UL, GridDirection::DR),
    (GridDirection::DL, GridDirection::UR),
    (GridDirection::DR, GridDirection::UL),
];

fn search_board_cross(grid: &Grid<char>, word: &[char]) -> usize {
    let mut hits = 0;

    let pivot = (word.len() - 1) / 2;
    let mid = word[pivot];
    let first: Vec<char> = word[0..(pivot + 1)].iter().rev().map(|s| *s).collect();
    let first = &first[..];
    let last = &word[pivot..];

    for i in grid {
        if i.value != mid { continue; }

        let mut point_hits = 0;
        for (start_dir, end_dir) in CROSS_DIRECTIONS {
            if search_word(grid, &i.point, start_dir, first) == 1 && search_word(grid, &i.point, end_dir, last) == 1 {
                point_hits += 1;
            }
        }

        if point_hits > 1 {
            hits += 1;
        }
    }

    hits
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("must specify input file");
    let file = io::BufReader::new(fs::File::open(input_filename).expect("could not open file"));
    let grid = Grid::<char>::chars_from_reader(file);

    let word = ['X', 'M', 'A', 'S'];
    let hits = search_board(&grid, &word);
    println!("Part 1 Total Hits: {}", hits);

    let word = ['M', 'A', 'S'];
    let hits = search_board_cross(&grid, &word);
    println!("Part 2 Total Hits: {}", hits);
}
