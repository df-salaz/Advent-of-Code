use std::env;
use std::fs;
use std::ops::ControlFlow;

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
            let middle = printing_update[printing_update.len()/2];
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
            let middle = corrected_update[corrected_update.len()/2];
            total += middle;
        };
    };

    println!("Part 2:");
    println!("{total}");
}

fn fix_update(printing_update: &Vec<usize>, ordering_rules: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut new_update = printing_update.clone();
    let mut i = 0;
    while i < printing_update.len() {
        for rule in ordering_rules.iter() {
            if rule[0] == printing_update[i] {

            }
        }
        i += 1;
    };
    new_update
}

fn check_update(printing_update: &Vec<usize>, ordering_rules: &Vec<Vec<usize>>) -> bool {
    let mut failed = false;
    for rule in ordering_rules.iter() {
        let good = check_rule(printing_update, &rule);

        if !good {
            failed = true;
        };
    }
    !failed
}

fn check_rule(printing_update: &Vec<usize>, rule: &Vec<usize>) -> bool {
    let before = rule[0];
    let after = rule[1];

    let before_index = printing_update.iter().position(|&page| page == before);
    match before_index {
        None => {
            return true;
        },
        Some(..) => {}
    };
    let after_index = printing_update.iter().position(|&page| page == after);
    match after_index {
        None => {
            return true;
        },
        Some(..) => {}
    };
    before_index <= after_index
}

fn get_vec_vec(section: &str, pattern: char) -> Vec<Vec<usize>> {
    let vec: Vec<Vec<usize>> = section.split('\n')
        .map(|string| string.trim())
        .map(|string: &str| {
            string.split(pattern)
                    .map(|number| number
                        .parse()
                        .unwrap()
                    )
                    .collect()
        }
        )
        .collect();
    vec
}

fn get_sections(input: &String) -> Vec<&str> {
    let sections: Vec<&str> = input.split("\n\n")
        .map(|string| string.trim())
        .collect();
    sections
}

