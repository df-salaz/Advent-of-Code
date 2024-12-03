use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply an input file");
    }
    let input_file_path = &args[1];
    let input = fs::read_to_string(input_file_path).unwrap();

    let split: Vec<&str> = input.split_whitespace().collect();

    let mut left_vec: Vec<i32> = vec![];
    let mut right_vec: Vec<i32> = vec![];
    for (i, val) in split.iter().enumerate() {
        if i % 2 == 0 {
            left_vec.push(val.parse::<i32>().unwrap());
        } else {
            right_vec.push(val.parse::<i32>().unwrap());
        }
    }
    assert_eq!(left_vec.len(), right_vec.len());
    left_vec.sort();
    right_vec.sort();

    let mut scores: Vec<i32> = vec![];
    for val in left_vec.iter() {
        let occurrences: i32 = right_vec.iter().filter(|&n| n == val).count().try_into().unwrap();
        scores.push(val * occurrences);
    }

    let sum: i32 = scores.iter().sum();
    println!("SOLUTION:");
    println!("{sum}");
}
