use std::collections::HashMap;
use shared::args::read_file_from_arg;


fn parse_data(data: String) -> (Vec<String>, Vec<String>) {
    let (towel_data, design_data) = data.split_once("\n\n").expect("could not parse file");

    let towels = towel_data.split(", ").map(|t| t.to_string()).collect();

    let designs = design_data.lines().map(|d| d.to_string()).collect();

    (towels, designs)
}

fn possible_patterns<'a>(design: &'a str, towels: &Vec<String>, cache: &mut HashMap<&'a str, usize>) -> usize {
    if design.len() == 0 {
        return 1;
    }

    if cache.contains_key(design) {
        return *cache.get(design).unwrap();
    }

    let mut possibilities = 0;
    for towel in towels {
        if design.starts_with(towel) {
            possibilities += possible_patterns(&design[towel.len()..], towels, cache);
        }
    }

    cache.insert(design, possibilities);

    possibilities
}


fn main() {
    let (towels, designs) = parse_data(read_file_from_arg(1));

    let total = designs.iter().filter(|&d| possible_patterns(d.as_str(), &towels, &mut HashMap::new()) > 0).count();
    println!("Part 1 Total Designs: {}", total);

    let total = designs.iter().map(|d| possible_patterns(d.as_str(), &towels, &mut HashMap::new())).sum::<usize>();
    println!("Part 2 Total Designs: {}", total);
}
