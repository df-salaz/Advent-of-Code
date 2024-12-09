use std::{env, fs};

fn main() {
    let input = process_input();

    let (fs_map, largest_id) = generate_filesystem(input);

    let contiguous = part_1(&fs_map);
    println!("Part 1: {}", get_checksum(contiguous));

    let defrag = part_2(&fs_map, largest_id);
    println!("Part 2: {}", get_checksum(defrag));
}

fn part_2(fs_map: &Vec<Option<i64>>, largest_id: i64) -> Vec<Option<i64>> {
    let mut filesystem: Vec<Option<i64>> = fs_map.clone();
    for id in (0..=largest_id).rev() {
        let size = filesystem
            .iter()
            .filter(|block| **block == Some(id))
            .count();
        match filesystem
            .iter()
            .enumerate()
            // Find the start index of a contiguous empty block where we fit
            // This is wildly inefficient and runs the whole check for every empty block
            .position(|(i, block)| {
                match block {
                    Some(_) => return false,
                    None => (),
                };
                for j in (i + 1)..(i + size) {
                    if j >= filesystem.len() { return false };
                    match filesystem[j] {
                        Some(_) => return false,
                        None => continue,
                    };
                };
                true
            }) {
                Some(empty_index) => {
                    let file_index = match filesystem
                        .iter()
                        .position(|block| *block == Some(id)) {
                            Some(v) => v,
                            None => continue,
                        };
                    // only move files left
                    if empty_index > file_index { continue };
                    for i in file_index..(file_index + size) {
                        filesystem[i] = None;
                    }
                    for i in empty_index..(empty_index + size) {
                        filesystem[i] = Some(id);
                    }
                },
                // do nothing if we don't fit anywhere
                None => continue,
            };
    };
    filesystem
}

fn part_1(fs_map: &Vec<Option<i64>>) -> Vec<Option<i64>> {
    let mut fs_map = fs_map.clone();
    loop {
        // look for an empty block
        let index = match fs_map.iter().position(|block| *block == None) {
            Some(val) => val,
            // if we don't find one, we have succeeded in making the filesystem contiguous
            None => break,
        };
        // move the last block to the empty block
        fs_map[index] = fs_map.pop().unwrap();
    }
    fs_map
}

fn generate_filesystem(input: String) -> (Vec<Option<i64>>, i64) {
    let mut fs_map: Vec<Option<i64>> = Vec::new();
    let mut id = 0;
    for (i, c) in input.chars().enumerate() {
        match i % 2 {
            0 => {
                fs_map.append(&mut vec![Some(id); c.to_digit(10).unwrap().try_into().unwrap()]);
                id += 1;
            },
            1 => fs_map.append(&mut vec![None; c.to_digit(10).unwrap().try_into().unwrap()]),
            _ => (),
        }
    }
    (fs_map, id - 1)
}

fn get_checksum(fs_map: Vec<Option<i64>>) -> i64 {
    let mut total: i64 = 0;
    let mut i: usize = 0;
    while i < fs_map.len() {
        match fs_map[i] {
            Some(_) => total += i as i64 * fs_map[i].unwrap(),
            None => (),
        }
        i += 1;
    }
    total
}

fn process_input() -> String {
    let filename = env::args()
        .last()
        .unwrap();
    fs::read_to_string(filename)
        .unwrap_or_default()
        .trim()
        .to_string()
}

