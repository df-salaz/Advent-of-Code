use std::env;
use std::fs;

use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::EnumCount;
use strum_macros::EnumIter;

fn main() {
    let matrix = match process_input() {
        Some(value) => value,
        None => return,
    };

    let mut total: i64 = 0;
    for line in matrix.iter() {
        if check_possible(&line) {
            total += line[0];
        };
    }

    println!("{total}");
}

fn check_possible(line: &[i64]) -> bool {
    let mut results: Vec<i64> = Vec::with_capacity(((line.len() - 1) - 1) ^ Operator::COUNT);

    populate_results(&mut results, &line[1..line.len()]);

    results.contains(&line[0])
}

fn populate_results(results: &mut Vec<i64>, values: &[i64]) {
    let mut sequence: Vec<Operator> = Vec::with_capacity(values.len() - 1);
    populate_helper(results, values, &mut sequence, values.len());
}

fn populate_helper(results: &mut Vec<i64>, values: &[i64], sequence: &mut Vec<Operator>, count: usize) {
    if count == 1 {
        results.push(apply_operators(sequence, values));
        return;
    }

    for operator in Operator::iter() {
        let mut new_sequence = sequence.clone();
        new_sequence.push(operator);
        populate_helper(results, values, &mut new_sequence, count - 1);
    };
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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumIter, EnumCount)]
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
                (lh.to_string() + rh.to_string().as_str()).parse().unwrap()
            },
        }
    }
}

