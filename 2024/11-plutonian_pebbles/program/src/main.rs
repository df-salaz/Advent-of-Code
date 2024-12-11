use std::{env, fs};

fn main() {
    let rocks = parse_input();

    part_1(&rocks);
}

fn part_1(rocks: &Rocks) {
    let mut rocks = rocks.clone();

    for _ in 0..25 {
        rocks.blink();
    }

    println!("{}", rocks.vec.len());
}

fn parse_input() -> Rocks {
    let args = env::args();
    let out_string = fs::read_to_string(args.last().unwrap()).unwrap();
    let vec = out_string.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    Rocks::new(vec)
}

#[derive(Clone)]
struct Rocks {
    vec: Vec<u64>,
}

impl Rocks {
    fn blink(&mut self) {
        let mut new_vec: Vec<u64> = Vec::new();

        for stone in self.vec.iter() {
            if *stone == 0 {
                new_vec.push(1);
                continue;
            }
            let count_digits = count_digits(*stone);
            match count_digits % 2 {
                0 => {
                    let mut left: u64 = *stone;
                    for _ in 0..(count_digits / 2) {
                        left /= 10;
                    }
                    new_vec.push(left);
                    let base: u64 = 10;
                    let factor = base.pow((count_digits / 2).try_into().unwrap());
                    new_vec.push(*stone - left * factor);
                },
                1 => {
                    new_vec.push(*stone * 2024);
                },
                _ => (),
            }
        }

        self.vec = new_vec;
    }

    fn new(vec: Vec<u64>) -> Self {
        Self { vec }
    }
}

fn print_vector(new_vec: &Vec<u64>) {
    for stone in new_vec.iter() {
        print!("{} ", stone);
    }
    println!();
}

fn count_digits(int: u64) -> u64 {
    if int == 0 { return 1 };
    let float: f64 = int as f64;

    float.log10().floor() as u64 + 1
}

