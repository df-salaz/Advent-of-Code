use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply an input file");
    }
    let input_file_path = &args[1];
    let input = fs::read_to_string(input_file_path).unwrap();
    let char_vec: Vec<char> = input.chars().collect();

    let mut i = 0;
    let mut total = 0;
    while i < char_vec.len() {
        let mut c = char_vec[i];

        if c != 'm' {
            i += 1;
            continue;
        }
        if input[i..i+4] != *"mul(" {
            i += 1;
            continue;
        }
        i += 4;

        let numstart = i;
        c = char_vec[i];
        while c.is_digit(10) {
            i += 1;
            c = char_vec[i];
        };
        if i - numstart > 3 {
            continue;
        };
        if c != ',' {
            i += 1;
            continue;
        }
        let num_a: usize = input[numstart..i].parse().unwrap();

        i += 1;

        let numstart = i;
        c = char_vec[i];
        while c.is_digit(10) {
            i += 1;
            c = char_vec[i];
        };
        if i - numstart > 3 {
            continue;
        };
        if c != ')' {
            i += 1;
            continue;
        }
        let num_b: usize = input[numstart..i].parse().unwrap();

        total += num_a * num_b;

        i += 1;
    };
    println!("{total}");
}