use std::fs;
use std::env;
use std::io;
use shared::grid::Grid;


fn search_board(grid: &Grid<char>, word: &[char]) -> usize {
    let mut hits = 0;

    // for (y, row) in grid.iter().enumerate() {
    //     for (x, _) in row.iter().enumerate() {
    //         // TODO
    //     }
    // }

    hits
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("must specify input file");
    let file = io::BufReader::new(fs::File::open(input_filename).expect("could not open file"));
    let grid = Grid::<char>::chars_from_reader(file);

    let word = ['X', 'M', 'A', 'S'];

    let hits = search_board(&grid, &word);
    println!("Total Hits: {}", hits);
}
