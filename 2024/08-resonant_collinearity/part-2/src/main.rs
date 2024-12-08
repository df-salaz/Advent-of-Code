use std::env;
use std::fs::read_to_string;

fn main() {
    let input_matrix = get_input_matrix();

    let mut total = 0;
    let mut anode_matrix: Vec<Vec<char>> = vec![vec!['.'; input_matrix[0].len()]; input_matrix.len()];
    for (y, line) in input_matrix.iter().enumerate() {
        let y = y as i32;
        for (x, node) in line.iter().enumerate() {
            let x = x as i32;
            match node {
                '.' => { continue },
                c => {
                    for (y1, line) in input_matrix.iter().enumerate() {
                        let y1 = y1 as i32;
                        for (x1, node) in line.iter().enumerate() {
                            let x1 = x1 as i32;
                            if x == x1 && y == y1 { continue };
                            if c == node {
                                place_anode(&mut anode_matrix, y, x, &mut total);
                                let x_dist = x1 - x;
                                let y_dist = y1 - y;
                                let mut anode_x = x - x_dist;
                                let mut anode_y = y - y_dist;
                                while !oob(anode_y, anode_x, &anode_matrix) {
                                    place_anode(&mut anode_matrix, anode_y, anode_x, &mut total);
                                    anode_y -= y_dist;
                                    anode_x -= x_dist;
                                }
                            }
                        }
                    }
                },
            }
        }
    }

    println!("{total}");
}

fn place_anode(anode_matrix: &mut Vec<Vec<char>>, anode_y: i32, anode_x: i32, total: &mut i32) {
    if !(anode_matrix[anode_y as usize][anode_x as usize] == '#') {
        anode_matrix[anode_y as usize][anode_x as usize] = '#';
        *total += 1;
    };
}

fn oob(y: i32, x: i32, matrix: &Vec<Vec<char>>) -> bool {
    y < 0 || y as usize >= matrix.len() || x < 0 || x as usize >= matrix[0].len()
}

fn get_input_matrix() -> Vec<Vec<char>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Please supply an input file"); }
    read_to_string(&args[1])
        .unwrap()
        .trim()
        .split('\n')
        .map(|string| { string.chars().collect() })
        .collect()
}

