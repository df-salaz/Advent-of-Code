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

    let ordering_rules = get_vec_of_lines(sections[0]);
    let print_updates = get_vec_of_lines(sections[1]);

    println!("{}\n{}", ordering_rules[0], print_updates[0]);
}

fn get_vec_of_lines(section: &str) -> Vec<&str> {
    let vec: Vec<&str> = section.split('\n')
        .map(|string| string.trim())
        .collect();
    vec
}

fn get_sections(input: &String) -> Vec<&str> {
    let sections: Vec<&str> = input.split("\n\n")
        .map(|string| string.trim())
        .collect();
    sections
}


