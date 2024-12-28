use std::collections::{HashMap, HashSet};
use shared::args::read_file_from_arg;


type Secret = u64;


fn next_secret(secret: Secret) -> Secret {
    let mut next = (secret ^ (secret * 64)) % 16777216;
    next = (next ^ (next / 32)) % 16777216;
    next = (next ^ (next * 2048)) % 16777216;
    next
}

fn part1(secrets: &Vec<Secret>) -> Secret {
    let mut sum = 0;

    for secret in secrets {
        let mut next = *secret;
        for _ in 0..2000 {
            next = next_secret(next);
        }
        sum += next
    }

    sum
}

fn part2(secrets: &Vec<Secret>) -> Secret {
    let mut patterns: HashMap<Vec<i64>, Secret> = HashMap::new();

    for secret in secrets {
        let mut numbers: Vec<Secret> = Vec::from_iter([*secret]);
        for _ in 0..2000 {
            numbers.push(next_secret(*numbers.last().unwrap()));
        }

        let diffs: Vec<i64> = numbers
            .windows(2)
            .map(|w| (w[1] % 10) as i64 - (w[0] % 10) as i64)
            .collect()
        ;

        let mut seen: HashSet<Vec<i64>> = HashSet::new();
        for i in 0..(numbers.len() - 4) {
            let pat = diffs[i..(i + 4)].to_vec();
            if seen.contains(&pat) {
                continue;
            }

            *patterns.entry(pat.clone()).or_insert(0) += numbers[i + 4] % 10;
            seen.insert(pat);
        }
    }

    *patterns.values().max().unwrap()
}


fn main() {
    let secrets: Vec<Secret> = read_file_from_arg(1)
        .lines()
        .map(|l| l.parse().expect("could not parse int"))
        .collect()
    ;

    println!("Part 1 Total Sum: {}", part1(&secrets));
    println!("Part 2 Most Bananas: {}", part2(&secrets));
}
