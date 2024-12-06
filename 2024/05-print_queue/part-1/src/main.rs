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

    for update in printing_updates.iter() {
        let mut failed = false;
        for rule in ordering_rules.iter() {
            let before = rule[0];
            let after = rule[1];

            let before_index = update.iter().position(|&page| page == before);
            match before_index {
                None => {
                    continue;
                },
                Some(..) => {}
            }
            let after_index = update.iter().position(|&page| page == after);
            match after_index {
                None => {
                    continue;
                },
                Some(..) => {}
            }

            if before_index > after_index {
                println!("Before: {}:{}; After: {}:{}",
                    before_index.unwrap(),
                    before,
                    after_index.unwrap(),
                    after
                );
                failed = true;
            }
        }
        if !failed {
            let middle = update[update.len()/2];
            total += middle;
        }
    }

    println!("{total}");
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


