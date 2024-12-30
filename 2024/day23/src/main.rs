use std::collections::HashMap;
use itertools::Itertools;
use shared::args::read_file_from_arg;


type Node<'a> = &'a str;
type Connection<'a> = (Node<'a>, Node<'a>);
type Network<'a> = HashMap<Node<'a>, Vec<Node<'a>>>;


fn parse_data<'a>(data: &'a str) -> Vec<Connection<'a>> {
    data.lines()
        .map(|l| {
            l.split_once('-').expect("could not parse pair")
        })
        .collect()
}

fn build_network<'a>(connections: &'a Vec<Connection>) -> Network<'a> {
    let mut network: Network<'a> = HashMap::new();

    for (a, b) in connections {
        network.entry(a).or_insert_with(|| Vec::new()).push(b);
        network.entry(b).or_insert_with(|| Vec::new()).push(a);
    }

    network
}

fn find_subnets<'a>(network: &'a Network, size: usize) -> Vec<Vec<Node<'a>>> {
    let mut subnets = Vec::new();

    'outer: for set in network.keys().combinations(size) {
        for pair in set.iter().combinations(2) {
            if !network[*pair[0]].contains(*pair[1]) {
                continue 'outer;
            }
        }

        subnets.push(set.iter().map(|&n| *n).collect());
    }

    subnets
}

fn part1(network: &Network) -> usize {
    find_subnets(&network, 3).iter()
        .filter(|&s| s.iter().any(|&n| n.starts_with('t')))
        .count()
}


fn main() {
    let data = read_file_from_arg(1);
    let connections = parse_data(&data);
    let network = build_network(&connections);

    println!("Part 1 Total Sets: {}", part1(&network));
}
