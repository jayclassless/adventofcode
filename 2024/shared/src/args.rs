use std::{env, fs};
use std::fs::File;
use std::io;


pub fn get_reader_for_arg(index: usize) -> io::BufReader<File> {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(index).expect("must specify input file");
    io::BufReader::new(File::open(input_filename).expect("could not open file"))
}

pub fn read_file_from_arg(index: usize) -> String {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(index).expect("must specify input file");
    fs::read_to_string(input_filename).expect("could not read file")
}
