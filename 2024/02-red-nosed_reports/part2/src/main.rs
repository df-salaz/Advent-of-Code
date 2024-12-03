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
        let mut report: Vec<i32> = vec![];
        for level in str_levels.iter() {
            report.push(level.parse().expect("Failed to parse to int"))
        }
        if check_report(&report) { safe_reports += 1; };
    };

    println!("Safe Reports: {safe_reports}");
}

fn check_report(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut smaller = report.clone();
        smaller.remove(i);
        let success = perform_checks(&smaller);
        if success { return true };
    };
    false
}

fn perform_checks(report: &Vec<i32>) -> bool {
    if report.len() == 0 { return true };

    let increasing: bool;

    if report.len() > 1 {
        if report[0] < report[1] { increasing = true }
        else if report[0] > report[1] { increasing = false }
        else { return false };
    } else {
        return false;
    }

    for (i, level) in report.iter().enumerate() {
        if i == 0 { continue; };
        // Distance
        if (*level - report[i-1]).abs() > 3 {
            return false
        };
        // inc/dec
        if increasing {
            if *level <= report[i-1] {
                return false
            };
        } else {
            if *level >= report[i-1] {
                return false
            }
        };
    };
    true
}

