use std::{collections::HashMap, env, fs, time::Instant};

fn main() {
    let rocks = parse_input();

    let now = Instant::now();

    part_1(rocks.clone());
    part_2(rocks);

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}

fn part_2(rocks: Rocks) {
    let mut rocks = rocks.clone();

    for _ in 0..75 {
        rocks.blink();
    }

    println!("{}", rocks.map.values().sum::<u64>());
}

fn part_1(rocks: Rocks) {
    let mut rocks = rocks.clone();

    for _ in 0..25 {
        rocks.blink();
    }

    println!("{}", rocks.map.values().sum::<u64>());
}

fn parse_input() -> Rocks {
    let args = env::args();
    let out_string = fs::read_to_string(args.last().unwrap()).unwrap();
    let vec: Vec<u64> = out_string.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut map = HashMap::new();
    for int in vec {
        match map.get(&int) {
            Some(_) => {
                *map.entry(int).or_default() += 1;
            },
            None => {
                map.insert(int, 1);
            },
        }
    }

    Rocks::new(map)
}

#[derive(Clone)]
struct Rocks {
    map: HashMap<u64, u64>,
}

impl Rocks {
    fn blink(&mut self) {
        let mut stones: HashMap<u64, u64> = HashMap::with_capacity(self.map.len());

        for (stone, count) in &self.map {
            match stone {
                0 => *stones.entry(1).or_default() += count,
                stone if stone.ilog10() % 2 == 1 => {
                    let halver: u64 = 10i64.pow((stone.ilog10() + 1) / 2).try_into().unwrap();
                    *stones.entry(stone % halver).or_default() += count;
                    *stones.entry(stone / halver).or_default() += count;
                }
                _ => *stones.entry(stone * 2024).or_default() += count,
            }
        }

        self.map = stones;
    }

    fn new(map: HashMap<u64, u64>) -> Self {
        Self {
            map,
        }
    }
}

