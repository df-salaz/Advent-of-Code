use std::env;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let matrix = match process_input() {
        Some(value) => value,
        None => return,
    };

    let mut handles = Vec::with_capacity(matrix.len());
    let counter: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));

    for i in 0..matrix.len() {
        let line = matrix[i].clone();
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            if check_possible(&line) {
                let mut total = counter.lock().unwrap();
                *total += line[0];
            };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap());
}

fn check_possible(line: &[i64]) -> bool {
    let values = &line[1..line.len()];
    let mut sequence: Vec<Operator> = Vec::new();
    return check_helper(
        values,
        &mut sequence,
        values.len(),
        line[0],
    );
}

fn check_helper(
    values: &[i64],
    sequence: &mut Vec<Operator>,
    count: usize,
    goal: i64,
) -> bool {
    if count == 1 {
        return apply_operators(sequence, values) == goal;
    }

    for operator in Operator::iter() {
        let mut new_sequence = sequence.clone();
        new_sequence.push(operator);
        let solution_found = check_helper(
            values,
            &mut new_sequence,
            count - 1,
            goal,
        );

        if solution_found { return true };
    };

    false
}

fn apply_operators(sequence: &mut Vec<Operator>, values: &[i64]) -> i64 {
    if sequence.len() == 0 {
        return values[0];
    }

    let operator = sequence.pop().unwrap();

    operator.apply(
        apply_operators(
            sequence,
            &values[0..values.len() - 1],
        ),
        values[values.len() - 1],
    )
}

fn process_input() -> Option<Vec<Vec<i64>>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please supply an input file");
        return None;
    }
    let input_file_path = &args[1];
    let input = fs::read_to_string(input_file_path)
        .unwrap()
        .lines()
        .map(|string| {
            string.split_whitespace()
                .map(|str| {
                    match str.chars().last().unwrap() {
                        '0'..='9' => str.parse().unwrap(),
                        _ => str[0..str.len() - 1].parse().unwrap(),
                    }
                })
            .collect()
        })
        .collect();
    Some(input)
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Operator {
    Add,
    Multiply,
    Concatinate,
}

impl Operator {
    pub fn apply(&self, lh: i64, rh: i64) -> i64 {
        match self {
            Operator::Add => lh + rh,
            Operator::Multiply => lh * rh,
            Operator::Concatinate => {
                format!("{lh}{rh}").parse().unwrap()
            },
        }
    }
}

