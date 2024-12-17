use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

type Page = u32;
type Update = Vec<Page>;

#[derive(Debug, PartialEq)]
struct OrderingRule {
    before: Page,
    after: Page,
}

fn parse_file(path: &str) -> (Vec<OrderingRule>, Vec<Update>) {
    let file = fs::File::open(path).expect("could not open file");
    let reader = io::BufReader::new(file);

    let mut orderings: Vec<OrderingRule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();

    let mut reading_updates = false;
    for line in reader.lines() {
        let value = line.expect("could not read line");

        if value == "" {
            reading_updates = true;
            continue;
        }

        if reading_updates {
            let u: Update = value.split(',').map(|v| v.parse().expect("could not parse update page")).collect();
            updates.push(u);
        } else {
            let mut parts = value.split('|');
            let o = OrderingRule {
                before: parts.next().unwrap().parse().expect("could not parse page"),
                after: parts.next().unwrap().parse().expect("could not parse page"),
            };
            orderings.push(o);
        }
    }

    (orderings, updates)
}

fn update_is_valid(update: &Update, orderings: &Vec<OrderingRule>) -> bool {
    for ordering in orderings {
        if let Some(bidx) = update.iter().position(|&p| p == ordering.before) {
            if let Some(aidx) = update.iter().position(|&p| p == ordering.after) {
                if bidx > aidx {
                    return false
                }
            }
        }
    }

    true
}

fn fix_update(update: &Update, orderings: &Vec<OrderingRule>) -> Update {
    let mut fixed = update.clone();

    let cmp= |a: &u32, b: &u32| {
        if orderings.contains(&OrderingRule { before: *a, after: *b }) {
            return Ordering::Less;
        } else if orderings.contains(&OrderingRule { before: *b, after: *a }) {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    };

    fixed.sort_by(cmp);
    fixed
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("must specify input file");

    let (orderings, updates) = parse_file(input_filename);

    let mut correct_sum = 0;
    let mut fixed_sum = 0;
    for update in updates {
        if update_is_valid(&update, &orderings) {
            correct_sum += update[(update.len() - 1) / 2];
        } else {
            let fixed_update = fix_update(&update, &orderings);
            fixed_sum += fixed_update[(update.len() - 1) / 2];
        }
    }

    println!("Part 1 Total Sum: {}", correct_sum);
    println!("Part 2 Total Sum: {}", fixed_sum);
}
