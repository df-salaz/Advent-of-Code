use std::{cell::RefCell, collections::HashMap, env, fs, rc::Rc};

static WIDTH: i64 = 101;
static HEIGHT: i64 = 103;

fn main() {
    let size: usize = (WIDTH * HEIGHT).try_into().unwrap();
    let map: Bathroom = Rc::new(RefCell::new(HashMap::with_capacity(size)));
    let robots = parse_input(map);

    let pt_1 = part_1(&robots);
    println!("Part 1: {pt_1}");

    let pt_2 = part_2(&robots);
    println!("Part 2: {pt_2}");
}

fn part_1(bots: &[Robot]) -> i64 {
    let mut bots = bots.to_owned();

    for bot in bots.iter_mut() {
        bot.exist();
        for _ in 0..100 {
            bot.move_bot();
        }
    }

    get_safety(bots[0].map.take())
}

fn part_2(bots: &[Robot]) -> i64 {
    let mut bots = bots.to_owned();

    for bot in bots.iter() {
        bot.exist();
    }

    let mut time = 0;
    loop {
        for bot in bots.iter_mut() {
            bot.move_bot();
        }
        time += 1;

        if easter_egg(&bots) {
            return time;
        }
    }
}

// check for when at least one fourth of the bots are surrounded on all sides by another bot
fn easter_egg(bots: &Vec<Robot>) -> bool {
    let touching: [(i64, i64); 8] = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, 1),
        (-1, -1),
        (1, -1),
    ];

    let mut total = 0;
    for bot in bots {
        let mut count = 0;
        for delta in touching {
            if bot
                .map
                .borrow()
                .get(&(bot.pos.0 + delta.0, bot.pos.1 + delta.1))
                .is_some()
            {
                count += 1
            }
        }
        if count == 8 {
            total += 1;
        }
    }

    if total > bots.len() / 4 {
        return true;
    }

    false
}

fn get_safety(bathroom: HashMap<Position, i64>) -> i64 {
    let (mut q0, mut q1, mut q2, mut q3) = (0, 0, 0, 0);
    let w = WIDTH / 2;
    let h = HEIGHT / 2;

    for (pos, bots) in bathroom {
        if pos.0 < w && pos.1 < h {
            q0 += bots;
        }
        if pos.0 > w && pos.1 > h {
            q1 += bots;
        }
        if pos.0 > w && pos.1 < h {
            q2 += bots;
        }
        if pos.0 < w && pos.1 > h {
            q3 += bots;
        }
    }

    q0 * q1 * q2 * q3
}

type Position = (i64, i64);

#[derive(Debug, Clone)]
pub struct Robot {
    pos: Position,
    vel: Position,
    map: Bathroom,
}

impl Robot {
    fn exist(&self) {
        let mut map = self.map.borrow_mut();
        *map.entry(self.pos).or_default() += 1;
    }

    fn move_bot(&mut self) {
        let mut map = self.map.borrow_mut();
        *map.entry(self.pos).or_default() -= 1;
        if *map.get(&self.pos).unwrap() == 0 {
            map.remove(&self.pos);
        }

        self.pos.0 = (self.pos.0 + self.vel.0).rem_euclid(WIDTH);
        self.pos.1 = (self.pos.1 + self.vel.1).rem_euclid(HEIGHT);

        *map.entry(self.pos).or_default() += 1;
    }
}

type Bathroom = Rc<RefCell<HashMap<Position, i64>>>;

fn parse_input(map: Bathroom) -> Vec<Robot> {
    fs::read_to_string(env::args().last().unwrap())
        .unwrap()
        .lines()
        .map(|line| {
            let mut p = None;
            let mut v = None;
            for attribute in line.split_whitespace() {
                let values = &attribute[2..];
                let (x, y) = values.split_at(values.chars().position(|c| c == ',').unwrap());
                let x: i64 = x.parse().unwrap();
                let y: i64 = y[1..].parse().unwrap();

                match &attribute[0..1] {
                    "p" => p = Some((x, y)),
                    "v" => v = Some((x, y)),
                    _ => (),
                }
            }
            let p = p.unwrap();
            let v = v.unwrap();
            Robot {
                pos: p,
                vel: v,
                map: Rc::clone(&map),
            }
        })
        .collect()
}
