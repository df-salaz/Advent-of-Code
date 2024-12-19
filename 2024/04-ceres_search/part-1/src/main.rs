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

    let mut found = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != 'X' {
                continue;
            }
            let room_north = y >= 3;
            let room_east = x < board_x - 3;
            let room_south = y < board_y - 4;
            let room_west = x >= 3;

            if room_north {
                check_north(lines, y, x, &mut found);
            };
            if room_north && room_east {
                check_northeast(lines, y, x, &mut found);
            };
            if room_east {
                check_east(line, x, &mut found);
            };
            if room_east && room_south {
                check_southeast(lines, y, x, &mut found);
            };
            if room_south {
                check_south(lines, y, x, &mut found);
            };
            if room_south && room_west {
                check_southwest(lines, y, x, &mut found);
            }
            if room_west {
                check_west(line, x, &mut found);
            };
            if room_west && room_north {
                check_northwest(lines, y, x, &mut found);
            };
        }
    }

    found
}

fn check_south(lines: &[&str], y: usize, x: usize, found: &mut usize) {
    let down = format!(
        "{}{}{}{}",
        lines[y].chars().nth(x).unwrap(),
        lines[y + 1].chars().nth(x).unwrap(),
        lines[y + 2].chars().nth(x).unwrap(),
        lines[y + 3].chars().nth(x).unwrap(),
    );
    if &down == "XMAS" {
        *found += 1;
        println!("south");
    };
}

fn check_southwest(lines: &[&str], y: usize, x: usize, found: &mut usize) {
    let down_left = format!(
        "{}{}{}{}",
        lines[y].chars().nth(x).unwrap(),
        lines[y + 1].chars().nth(x - 1).unwrap(),
        lines[y + 2].chars().nth(x - 2).unwrap(),
        lines[y + 3].chars().nth(x - 3).unwrap(),
    );
    if &down_left == "XMAS" {
        *found += 1;
        println!("southwest");
    }
}

fn check_southeast(lines: &[&str], y: usize, x: usize, found: &mut usize) {
    let down_right = format!(
        "{}{}{}{}",
        lines[y].chars().nth(x).unwrap(),
        lines[y + 1].chars().nth(x + 1).unwrap(),
        lines[y + 2].chars().nth(x + 2).unwrap(),
        lines[y + 3].chars().nth(x + 3).unwrap(),
    );
    if &down_right == "XMAS" {
        *found += 1;
        println!("southeast");
    }
}

fn check_northeast(lines: &[&str], y: usize, x: usize, found: &mut usize) {
    let up_right = format!(
        "{}{}{}{}",
        lines[y].chars().nth(x).unwrap(),
        lines[y - 1].chars().nth(x + 1).unwrap(),
        lines[y - 2].chars().nth(x + 2).unwrap(),
        lines[y - 3].chars().nth(x + 3).unwrap(),
    );
    if &up_right == "XMAS" {
        *found += 1;
        println!("northeast");
    };
}

fn check_northwest(lines: &[&str], y: usize, x: usize, found: &mut usize) {
    let up_left = format!(
        "{}{}{}{}",
        lines[y].chars().nth(x).unwrap(),
        lines[y - 1].chars().nth(x - 1).unwrap(),
        lines[y - 2].chars().nth(x - 2).unwrap(),
        lines[y - 3].chars().nth(x - 3).unwrap(),
    );
    if &up_left == "XMAS" {
        *found += 1;
        println!("northwest");
    };
}

fn check_north(lines: &[&str], y: usize, x: usize, found: &mut usize) {
    let up = format!(
        "{}{}{}{}",
        lines[y].chars().nth(x).unwrap(),
        lines[y - 1].chars().nth(x).unwrap(),
        lines[y - 2].chars().nth(x).unwrap(),
        lines[y - 3].chars().nth(x).unwrap(),
    );
    if &up == "XMAS" {
        *found += 1;
        println!("north");
    };
}

fn check_east(line: &&str, x: usize, found: &mut usize) {
    if &line[x..x + 4] == "XMAS" {
        *found += 1;
        println!("east");
    }
}

fn check_west(line: &str, x: usize, found: &mut usize) {
    if &line[x - 3..x + 1] == "SAMX" {
        *found += 1;
        println!("west");
    }
}
