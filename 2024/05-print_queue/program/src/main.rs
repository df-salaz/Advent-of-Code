use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please supply an input file");
        return;
    }
    let input_file_path = &args[1];
    let input = fs::read_to_string(input_file_path).unwrap();

    let sections = get_sections(&input);

    let ordering_rules = get_vec_vec(sections[0], '|');
    let printing_updates = get_vec_vec(sections[1], ',');

    let mut total = 0;

    for printing_update in printing_updates.iter() {
        let success = check_update(printing_update, &ordering_rules);
        if success {
            let middle = printing_update[printing_update.len() / 2];
            total += middle;
        }
    }

    println!("Part 1:");
    println!("{total}");

    total = 0;
    for printing_update in printing_updates.iter() {
        let success = check_update(printing_update, &ordering_rules);
        if !success {
            let corrected_update = fix_update(printing_update, &ordering_rules);
            let middle = corrected_update[corrected_update.len() / 2];
            total += middle;
        };
    }

    println!("Part 2:");
    println!("{total}");
}

fn fix_update(printing_update: &[i32], ordering_rules: &[Vec<i32>]) -> Vec<i32> {
    let mut new_update = printing_update.to_owned();
    while !check_update(&new_update, ordering_rules) {
        let mut i = 0;
        while i < new_update.len() {
            for rule in ordering_rules.iter() {
                if rule[0] != new_update[i] {
                    continue;
                };
                if check_rule(&new_update, rule) {
                    continue;
                };
                let after = match new_update.iter().position(|&page| page == rule[1]) {
                    None => {
                        continue;
                    }
                    Some(position) => position,
                };
                new_update.swap(i, after);
            }
            i += 1;
        }
    }
    new_update.to_vec()
}

fn check_update(printing_update: &[i32], ordering_rules: &[Vec<i32>]) -> bool {
    let mut failed = false;
    for rule in ordering_rules.iter() {
        let good = check_rule(printing_update, rule);

        if !good {
            failed = true;
        };
    }
    !failed
}

fn check_rule(printing_update: &[i32], rule: &[i32]) -> bool {
    let before = rule[0];
    let after = rule[1];

    let before_index = printing_update.iter().position(|&page| page == before);
    if before_index.is_none() {
        return true;
    }
    let after_index = printing_update.iter().position(|&page| page == after);
    if after_index.is_none() {
        return true;
    };
    before_index <= after_index
}

fn get_vec_vec(section: &str, pattern: char) -> Vec<Vec<i32>> {
    let vec: Vec<Vec<i32>> = section
        .split('\n')
        .map(|string| string.trim())
        .map(|string: &str| {
            string
                .split(pattern)
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();
    vec
}

fn get_sections(input: &str) -> Vec<&str> {
    let sections: Vec<&str> = input.split("\n\n").map(|string| string.trim()).collect();
    sections
}
