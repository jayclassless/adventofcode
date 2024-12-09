use std::fs;
use std::env;
use std::io;
use std::io::BufRead;

use shared::Point;


type Board = Vec<Vec<char>>;


fn search_board(board: &Board, word: &[char]) -> usize {
    let mut hits = 0;

    for (y, row) in board.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            // TODO
        }
    }

    hits
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("must specify input file");
    let file = io::BufReader::new(fs::File::open(input_filename).expect("could not open file"));
    let mut board: Board = Vec::new();
    for line in file.lines() {
        let l: Vec<char> = line.expect("could not parse line").chars().collect();
        board.push(l);
    }

    let word = ['X', 'M', 'A', 'S'];

    let hits = search_board(&board, &word);
    println!("Total Hits: {}", hits);
}
