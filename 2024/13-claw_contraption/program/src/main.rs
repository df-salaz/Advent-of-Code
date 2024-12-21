use std::{env, fs};

fn main() {
    let machines = parse_input();

    let pt_1: i64 = part_1(&machines);
    println!("Part 1: {pt_1}");

    let pt_2: i64 = part_2(&machines);
    println!("Part 2: {pt_2}");
}

fn part_1(machines: &[ClawMachine]) -> i64 {
    machines
        .iter()
        .filter_map(press_buttons)
        .map(|(a, b)| 3 * a + b)
        .sum()
}

fn part_2(machines: &[ClawMachine]) -> i64 {
    machines
        .to_owned()
        .iter_mut()
        .filter_map(|machine| {
            machine.prize.x += 10_000_000_000_000;
            machine.prize.y += 10_000_000_000_000;
            press_buttons(machine)
        })
        .map(|(a, b)| 3 * a + b)
        .sum()
}

#[derive(Clone, Copy)]
pub struct Position {
    x: i64,
    y: i64,
}
#[derive(Clone, Copy)]
pub struct Button {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
pub struct ClawMachine {
    prize: Position,
    a: Button,
    b: Button,
}

impl ClawMachine {
    fn new(prize: Position, a: Button, b: Button) -> Self {
        Self { prize, a, b }
    }
}

fn parse_input() -> Vec<ClawMachine> {
    let input_string = fs::read_to_string(env::args().last().unwrap()).unwrap();
    let machine_descriptions: Vec<&str> = input_string.split("\n\n").map(str::trim).collect();

    let mut claw_machines = Vec::with_capacity(machine_descriptions.len());
    for description in machine_descriptions {
        let mut a_button = None;
        let mut b_button = None;
        let mut prize = None;

        for line in description.lines() {
            match &line[0..10] {
                "Button A: " => {
                    let position = parse_position(line);
                    a_button = Some(Button {
                        x: position.x,
                        y: position.y,
                    });
                }
                "Button B: " => {
                    let position = parse_position(line);
                    b_button = Some(Button {
                        x: position.x,
                        y: position.y,
                    });
                }
                _ if &line[0..7] == "Prize: " => {
                    prize = Some(parse_position(line));
                }
                _ => {}
            }
        }

        let new_machine = ClawMachine::new(prize.unwrap(), a_button.unwrap(), b_button.unwrap());
        claw_machines.push(new_machine);
    }
    claw_machines
}

fn parse_position(line: &str) -> Position {
    let comma = line.chars().position(|c| c == ',').unwrap();
    let (lhs, rhs) = line.split_at(comma);

    let first = lhs.chars().position(|c| c.is_ascii_digit()).unwrap();
    let slice_x = &lhs[first..];
    let x: i64 = slice_x.parse().unwrap();

    let first = rhs.chars().position(|c| c.is_ascii_digit()).unwrap();
    let slice_y = &rhs[first..];
    let y: i64 = slice_y.parse().unwrap();

    Position { x, y }
}

fn press_buttons(cm: &ClawMachine) -> Option<(i64, i64)> {
    let a = (cm.prize.x * cm.b.y - cm.prize.y * cm.b.x) / (cm.a.x * cm.b.y - cm.a.y * cm.b.x);
    let b = (-cm.prize.x * cm.a.y + cm.prize.y * cm.a.x) / (cm.a.x * cm.b.y - cm.a.y * cm.b.x);

    if cm.a.x * a + cm.b.x * b == cm.prize.x && cm.a.y * a + cm.b.y * b == cm.prize.y {
        Some((a, b))
    } else {
        None
    }
}
