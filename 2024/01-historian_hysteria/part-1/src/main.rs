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

    let mut distances: Vec<i32> = vec![];
    for (i, left) in left_vec.iter().enumerate() {
        let right = right_vec.get(i).unwrap();
        let distance = (left - right).abs();
        distances.push(distance);
    }

    let sum: i32 = distances.iter().sum();
    println!("SOLUTION:");
    println!("{sum}");
}

