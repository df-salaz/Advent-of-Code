use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply an input file");
    }
    let input_file_path = &args[1];
    let input = fs::read_to_string(input_file_path).unwrap();

    let lines: Vec<&str> = input.split('\n').collect();

    let found = find(&lines);
    println!("{found}");
}

fn find(lines: &Vec<&str>) -> usize {
    let board_x = lines[0].len();
    let board_y = lines.len();

    println!("Board dimensions: {board_x} x {board_y}");

    let mut found = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != 'A' {
                continue;
            }
            if x >= board_x - 1 || x < 1 {
                continue;
            }
            if y >= board_y - 2 || y < 1 {
                continue;
            }

            if
                check_northeast(lines, y, x) && check_southeast(lines, y, x) ||
                check_northwest(lines, y, x) && check_southwest(lines, y, x) ||
                check_southwest(lines, y, x) && check_southeast(lines, y, x) ||
                check_northwest(lines, y, x) && check_northeast(lines, y, x) {
                    found += 1;
            }
        };
    };

    found
}

fn check_southwest(lines: &[&str], y: usize, x: usize) -> bool {
    let down_left = format!(
        "{}{}{}",
        lines[y-1].chars().nth(x+1).unwrap(),
        lines[y].chars().nth(x).unwrap(),
        lines[y+1].chars().nth(x-1).unwrap(),
    );
    if &down_left == "MAS" {
        return true;
    }
    false
}

fn check_southeast(lines: &[&str], y: usize, x: usize) -> bool {
    let down_right = format!(
        "{}{}{}",
        lines[y-1].chars().nth(x-1).unwrap(),
        lines[y].chars().nth(x).unwrap(),
        lines[y+1].chars().nth(x+1).unwrap(),
    );
    if &down_right == "MAS" {
        return true;
    }
    false
}

fn check_northeast(lines: &Vec<&str>, y: usize, x: usize) -> bool {
    let up_right = format!(
        "{}{}{}",
        lines[y+1].chars().nth(x-1).unwrap(),
        lines[y].chars().nth(x).unwrap(),
        lines[y-1].chars().nth(x+1).unwrap(),
    );
    if &up_right == "MAS" {
        return true
    };
    false
}

fn check_northwest(lines: &[&str], y: usize, x: usize) -> bool {
    let up_left = format!(
        "{}{}{}",
        lines[y+1].chars().nth(x+1).unwrap(),
        lines[y].chars().nth(x).unwrap(),
        lines[y-1].chars().nth(x-1).unwrap(),
    );
    if &up_left == "MAS" {
        return true
    };
    false
}

