use std::env;
use std::fs;
use std::usize;

fn main() {
    let input = match process_input() {
        Some(value) => value,
        None => return,
    };

    let field = get_2d_char_arry(input);

    let mut agent =
        match find_agent(field) {
            Some(value) => value,
            None => {
                println!("Could not find agent.");
                return
            },
        };

    while agent.move_agent() && !agent.in_loop {}
    let field = agent.field;

    let mut count = 0;
    for line in field.iter() {
        for c in line.iter() {
            if *c == 'X' {
                count += 1;
            };
        };
    };

    println!("Part 1:");
    println!("{count}");
}

fn process_input() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please supply an input file");
        return None;
    }
    let input_file_path = &args[1];
    let input = fs::read_to_string(input_file_path).unwrap();
    Some(input)
}

fn find_agent(field: Vec<Vec<char>>) -> Option<Agent> {
    let (mut agent_y, mut agent_x) = (None, None);
    for (y, line) in field.iter().enumerate() {
        let find = line.iter().position(|ch| *ch == '^');
        match find {
            Some(x) => {
                (agent_y, agent_x) = (Some(x as i32), Some(y as i32));
                break
            },
            None => continue,
        }
    }
    let agent =
        match {
            let agent;
            match agent_x {
                Some(_) => {
                    agent = Agent::new(
                        (agent_x.unwrap(), agent_y.unwrap()),
                        Facing::North,
                        field,
                    )
                },
                None => {
                    return None;
                },
            }
            Some(agent)
        } {
            Some(agent) => agent,
            None => {
                return None;
            },
        };
    Some(agent)
}

fn get_2d_char_arry(input: String) -> Vec<Vec<char>> {
    let field: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|string| {
            string.chars()
                .collect()
        })
        .collect();
    field
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Facing {
    North,
    East,
    South,
    West
}

struct Agent {
    location: (i32, i32),
    facing: Facing,
    field: Vec<Vec<char>>,
    memory: Vec<Vec<Option<((i32, i32), Facing)>>>,
    in_loop: bool,
}

impl Agent {
    fn new(location: (i32, i32), facing: Facing, field: Vec<Vec<char>>) -> Agent {
        let mut memory: Vec<Vec<Option<((i32, i32), Facing)>>> = Vec::with_capacity(field.len());
        for i in 0..field.len() {
            memory.push(
                Vec::with_capacity(field[0].len())
            );
            for _ in 0..field[0].len() {
                memory[i].push(None);
            }
        };

        let new_agent = Agent {
            location,
            facing,
            field,
            memory,
            in_loop: false,
        };
        new_agent
    }

    fn move_agent(&mut self) -> bool {
        let direction;

        match &self.facing {
            Facing::North => direction = (-1, 0),
            Facing::South => direction = (1, 0),
            Facing::East => direction = (0, 1),
            Facing::West => direction = (0, -1),
        }

        let loc = self.location;
        self.field[loc.0 as usize][loc.1 as usize] = 'X';

        let new_loc = (
            self.location.0 + direction.0,
            self.location.1 + direction.1,
        );

        self.memory[loc.0 as usize][loc.1 as usize] = Some((loc, self.facing));


        let loc = &new_loc;
        let bounds_y = self.field.len() as i32;
        let bounds_x = self.field[0].len() as i32;
        if loc.0 >= bounds_y || loc.0 < 0 || loc.1 >= bounds_x || loc.1 < 0
        { return false };

        if self.field[new_loc.0 as usize][new_loc.1 as usize] == '#' {
            self.facing = match &self.facing {
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North,
            };

            return self.move_agent();
        }

        match self.memory[new_loc.0 as usize][new_loc.1 as usize] {
            Some(state) => {
                if state == (new_loc, self.facing) {
                    self.in_loop = true;
                }
            },
            None => {},
        }

        self.location = new_loc;
        true
    }
}

