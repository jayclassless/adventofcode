use std::collections::HashMap;
use itertools::Itertools;
use shared::args::get_reader_for_arg;
use shared::grid::Grid;
use shared::point::Point;


type Maze = Grid<char>;

const START: char = 'S';
const END: char = 'E';
const EMPTY: char = '.';


fn score_path(maze: &Maze) -> HashMap<Point, isize> {
    let mut scores = HashMap::new();

    let mut point = maze.find(START).expect("could not find start");
    let mut score = 0;
    scores.insert(point, score);

    loop {
        for neighbor in point.direct_neighbors() {
            if scores.contains_key(&neighbor) {
                continue;
            }

            if let Some(contents) = maze.get_point(&neighbor) {
                if contents == EMPTY || contents == END {
                    score += 1;
                    scores.insert(neighbor, score);
                    point = neighbor;

                    if contents == END {
                        return scores;
                    }

                    break;
                }
            }
        }
    }
}

fn count_cheats<F>(scores: &HashMap<Point, isize>, f: F) -> usize
where 
    F: Fn(isize) -> bool
{
    let mut cheats: HashMap<isize, usize> = HashMap::new();

    for combo in scores.keys().sorted_by_key(|&k| scores[k]).combinations(2) {
        let dist = combo[0].distance(*combo[1]);

        if f(dist) {
            let score = scores[combo[1]] - scores[combo[0]] - dist;
            if score > 0 {
                *cheats.entry(score).or_insert(0) += 1;
            }
        }
    }

    cheats.iter().filter(|(&k, _)| k >= 100).map(|(_, v)| *v as usize).sum()
}


fn main() {
    let file = get_reader_for_arg(1);

    let maze: Maze = Grid::<char>::chars_from_reader(file);
    let scores = score_path(&maze);

    let cheats = count_cheats(&scores, |d| d == 2);
    println!("Part 1 Cheats: {:?}", cheats);

    let cheats = count_cheats(&scores, |d| d < 21);
    println!("Part 2 Cheats: {:?}", cheats);
}
