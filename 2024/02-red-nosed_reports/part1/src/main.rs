use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply an input file");
    }
    let input_file_path = &args[1];
    let input = fs::read_to_string(input_file_path).unwrap();

    let reports: Vec<&str> = input.split('\n').collect();
    let mut safe_reports: u32 = 0;

    for report in reports.iter() {
        let str_levels: Vec<&str> = report.trim().split_whitespace().collect();
        let mut levels: Vec<i32> = vec![];
        for level in str_levels.iter() {
            levels.push(level.parse().expect("Failed to parse to int"))
        }
        if check_inc(&levels) && check_dist(&levels) { safe_reports += 1; };
    }
}

fn check_inc(levels: &Vec<i32>) -> bool {

}

fn check_dist(levels: &Vec<i32>) -> bool {

}
