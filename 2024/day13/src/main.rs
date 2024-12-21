use std::env;
use std::fs;
use shared::point::Point;


#[derive(Debug)]
struct Machine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}


fn parse_button(line: &str) -> Point {
    let (_, defn) = line.split_once(": ").expect("could not parse button");
    let (x_defn, y_defn) = defn.split_once(", ").expect("could not parse button");

    Point::new(x_defn[2..].parse().expect("bad x"), y_defn[2..].parse().expect("bad y"))
}

fn parse_file(path: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    
    let data = fs::read_to_string(path).expect("could not read file");
    for chunk in data.lines().collect::<Vec<&str>>().chunks(4) {
        let button_a = parse_button(chunk[0]);
        let button_b = parse_button(chunk[1]);
        let prize = parse_button(chunk[2]);

        machines.push(Machine { button_a, button_b, prize });
    }

    machines
}

/*
px = A*ax + B*bx
py = A*ay + B*by

cramer's rule -->

A = (px*by - py*bx) / (ax*by - ay*bx)
B = (ax*py - ay*px) / (ax*by - ay*bx)
*/
fn cost(machine: &Machine, offset: isize) -> Option<isize> {
    let prize = machine.prize + Point::new(offset, offset);

    let denom = (machine.button_a.x * machine.button_b.y) - (machine.button_a.y * machine.button_b.x);
    let a = ((prize.x * machine.button_b.y) - (prize.y * machine.button_b.x)) / denom;
    let b = ((machine.button_a.x * prize.y) - (machine.button_a.y * prize.x)) / denom;

    if (((machine.button_a.x * a) + (machine.button_b.x * b)) == prize.x) && (((machine.button_a.y * a) + (machine.button_b.y * b)) == prize.y) {
        Some(a * 3 + b)
    } else {
        None
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("must specify input file");

    let machines = parse_file(input_filename);

    let total: isize = machines.iter().map(|m| cost(m, 0).unwrap_or(0)).sum();
    println!("Part 1 Cost: {}", total);

    let total: isize = machines.iter().map(|m| cost(m, 10000000000000).unwrap_or(0)).sum();
    println!("Part 2 Cost: {}", total);
}
