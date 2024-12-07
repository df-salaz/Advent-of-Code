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
}

