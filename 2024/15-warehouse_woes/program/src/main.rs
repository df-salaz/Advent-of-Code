use core::panic;
use std::{collections::HashMap, env, fs};

fn main() {
    let (warehouse, moves) = parse_input_1();

    let pt1 = part_1(&warehouse, &moves);
    println!("Part 1: {pt1}");

    let (warehouse, moves) = parse_input_2();

    let pt2 = part_2(&warehouse, &moves);
    println!("Part 2: {pt2}");
}

fn part_1(warehouse: &Warehouse1, moves: &[Direction]) -> i64 {
    let mut warehouse = warehouse.to_owned();

    for mve in moves {
        warehouse.traverse(*mve);
    }

    warehouse.print();
    warehouse
        .map
        .iter()
        .filter(|(_, t)| **t == Tile1::Box)
        .map(|((x, y), _)| {
            let x: i64 = (*x).try_into().unwrap();
            let y: i64 = (*y).try_into().unwrap();
            100 * y + x
        })
        .sum()
}

fn part_2(warehouse: &Warehouse2, moves: &[Direction]) -> i64 {
    let mut warehouse = warehouse.to_owned();

    for mve in moves {
        warehouse.traverse(*mve);
    }

    warehouse.print();
    warehouse
        .map
        .iter()
        .filter(|(_, t)| **t == Tile2::BoxL)
        .map(|((x, y), _)| {
            let x: i64 = (*x).try_into().unwrap();
            let y: i64 = (*y).try_into().unwrap();
            100 * y + x
        })
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile1 {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile2 {
    Empty,
    Wall,
    BoxL,
    BoxR,
    Robot,
}

impl Tile1 {
    fn from(c: char) -> Tile1 {
        match c {
            '.' => Tile1::Empty,
            '#' => Tile1::Wall,
            'O' => Tile1::Box,
            '@' => Tile1::Robot,
            _ => panic!("Illegal tile character: {c}"),
        }
    }

    fn print(self) {
        match self {
            Tile1::Empty => print!(" "),
            Tile1::Wall => print!("█"),
            Tile1::Box => print!("▒"),
            Tile1::Robot => print!("◆"),
        }
    }
}

impl Tile2 {
    fn from(c: char) -> Tile2 {
        match c {
            '.' => Tile2::Empty,
            '#' => Tile2::Wall,
            'O' => Tile2::BoxL,
            '@' => Tile2::Robot,
            _ => panic!("Illegal tile character: {c}"),
        }
    }

    fn print(self) {
        match self {
            Tile2::Empty => print!(" "),
            Tile2::Wall => print!("█"),
            Tile2::BoxL => print!("["),
            Tile2::BoxR => print!("]"),
            Tile2::Robot => print!("◆"),
        }
    }

    fn is_box(self) -> bool {
        self == Tile2::BoxL || self == Tile2::BoxR
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Illegal move character: {c}"),
        }
    }

    fn about(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }
}

type Position = (usize, usize);

#[derive(Clone)]
struct Warehouse1 {
    map: HashMap<Position, Tile1>,
    width: usize,
    height: usize,
    robot_pos: Position,
}

#[derive(Clone)]
struct Warehouse2 {
    map: HashMap<Position, Tile2>,
    width: usize,
    height: usize,
    robot_pos: Position,
}

impl Warehouse1 {
    fn new(map: HashMap<Position, Tile1>, width: usize, height: usize) -> Warehouse1 {
        assert!(
            map.iter().filter(|(_, t)| **t == Tile1::Robot).count() == 1,
            "Input does not contain one robot"
        );
        let (x, y) = *map.iter().find(|(_, t)| **t == Tile1::Robot).unwrap().0;

        Warehouse1 {
            map,
            width,
            height,
            robot_pos: (x, y),
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.map.get(&(x, y)).unwrap().print();
            }
            println!();
        }
    }

    fn traverse(&mut self, dir: Direction) {
        self.move_tile(self.robot_pos, dir);
    }

    fn move_tile(&mut self, pos: Position, dir: Direction) {
        let neighbor = dir.about(pos);

        match self.map.get(&neighbor) {
            Some(t) => match t {
                Tile1::Box => {
                    self.move_tile(neighbor, dir);
                    if *self.map.get(&neighbor).unwrap() == Tile1::Empty {
                        self.move_tile(pos, dir);
                    }
                }
                Tile1::Empty => {
                    let current = *self.map.get(&pos).unwrap();
                    if current == Tile1::Robot {
                        self.robot_pos = neighbor;
                    }
                    *self.map.get_mut(&neighbor).unwrap() = current;
                    *self.map.get_mut(&pos).unwrap() = Tile1::Empty;
                }
                _ => {}
            },
            None => unreachable!(),
        };
    }
}

impl Warehouse2 {
    fn new(map: HashMap<Position, Tile2>, width: usize, height: usize) -> Warehouse2 {
        assert!(
            map.iter().filter(|(_, t)| **t == Tile2::Robot).count() == 1,
            "Input does not contain one robot"
        );
        let (x, y) = *map.iter().find(|(_, t)| **t == Tile2::Robot).unwrap().0;

        Warehouse2 {
            map,
            width,
            height,
            robot_pos: (x, y),
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.map.get(&(x, y)).unwrap().print();
            }
            println!();
        }
    }

    fn traverse(&mut self, dir: Direction) {
        self.move_tile(self.robot_pos, dir);
    }

    fn move_tile(&mut self, pos: Position, dir: Direction) -> bool {
        let goal = dir.about(pos);

        let result = match self.map.get(&goal) {
            Some(goal_tile) => match goal_tile {
                Tile2::Empty => {
                    let current = *self.map.get(&pos).unwrap();
                    if current == Tile2::Robot {
                        self.robot_pos = goal;
                    }
                    *self.map.get_mut(&goal).unwrap() = current;
                    *self.map.get_mut(&pos).unwrap() = Tile2::Empty;
                    true
                }
                _ if goal_tile.is_box() => {
                    // same logic as before when moving left or right
                    if dir == Direction::Left || dir == Direction::Right {
                        self.move_tile(goal, dir);
                        if *self.map.get(&goal).unwrap() == Tile2::Empty {
                            self.move_tile(pos, dir);
                            return true;
                        }
                        return false;
                    }

                    let current_state = self.map.clone();

                    let goal_other_half = match goal_tile {
                        Tile2::BoxL => (goal.0 + 1, goal.1),
                        _ => (goal.0 - 1, goal.1),
                    };

                    // attempt to move both halves of the boxe we're trying to push
                    if !self.move_tile(goal, dir) {
                        self.map = current_state;
                        return false;
                    }
                    if !self.move_tile(goal_other_half, dir) {
                        self.map = current_state;
                        return false;
                    }

                    // we moved the box in front of us and can now attempt to take its place
                    if !self.move_tile(pos, dir) {
                        self.map = current_state;
                        return false;
                    }

                    // if I'm a box, try to move my other half
                    let me_other_hal = match self.map.get(&pos).unwrap() {
                        Tile2::BoxL => (pos.0 + 1, pos.1),
                        Tile2::BoxR => (pos.0 - 1, pos.1),
                        // I'm not a box, so let's stop trying to do things
                        _ => return true,
                    };
                    if !self.move_tile(me_other_hal, dir) {
                        self.map = current_state;
                        return false;
                    }

                    true
                }
                _ => false,
            },
            None => unreachable!(),
        };

        result
    }
}

fn collect_moves(moves_str: &str) -> Vec<Direction> {
    let moves_vec: Vec<Direction> = moves_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Direction::from)
        .collect();
    moves_vec
}

fn parse_input_1() -> (Warehouse1, Vec<Direction>) {
    let in_string = fs::read_to_string(env::args().last().unwrap_or_default()).unwrap_or_default();
    let (map_str, moves_str) = in_string.split_at(in_string.find("\n\n").unwrap_or_default());

    let map_vec: Vec<Vec<Tile1>> = map_str
        .trim()
        .lines()
        .map(|line| line.chars().map(Tile1::from).collect())
        .collect();

    let moves_vec = collect_moves(moves_str);

    assert!(
        !(map_vec.is_empty() || moves_vec.is_empty()),
        "Please provide a proper input file."
    );

    let mut map = HashMap::new();
    for (y, line) in map_vec.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            map.insert((x, y), *tile);
        }
    }

    (
        Warehouse1::new(map, map_vec.first().unwrap().len(), map_vec.len()),
        moves_vec,
    )
}

fn parse_input_2() -> (Warehouse2, Vec<Direction>) {
    let in_string = fs::read_to_string(env::args().last().unwrap_or_default()).unwrap_or_default();
    let (map_str, moves_str) = in_string.split_at(in_string.find("\n\n").unwrap_or_default());

    let map_vec: Vec<Vec<Tile2>> = map_str
        .trim()
        .lines()
        .map(|line| line.chars().map(Tile2::from).collect())
        .collect();

    let moves_vec = collect_moves(moves_str);

    let mut map = HashMap::new();
    for (y, line) in map_vec.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            map.insert((x * 2, y), *tile);
            map.insert(
                (x * 2 + 1, y),
                match tile {
                    Tile2::Robot | Tile2::Empty => Tile2::Empty,
                    Tile2::Wall => Tile2::Wall,
                    Tile2::BoxL => Tile2::BoxR,
                    Tile2::BoxR => unreachable!(),
                },
            );
        }
    }

    (
        Warehouse2::new(map, map_vec.first().unwrap().len() * 2, map_vec.len()),
        moves_vec,
    )
}
