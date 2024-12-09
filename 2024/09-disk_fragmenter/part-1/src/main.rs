use std::{env, fs::read_to_string};

fn main() {
    let input = process_input();

    let fs_map = generate_filesystem(input);

    let contiguous = part_1(fs_map.clone());
    let contiguous_checksum = get_checksum(contiguous);
    println!("{contiguous_checksum}");

    //let defrag = part_2(fs_map.clone());
    //let defrag_checksum = get_checksum(defrag);
    //println!("{defrag_checksum}");
}

fn part_2(fs_map: Vec<Option<i64>>) -> Vec<Option<i64>> {
    todo!()
}

fn part_1(fs_map: Vec<Option<i64>>) -> Vec<Option<i64>> {
    let mut fs_map = Box::new(fs_map);
    for block in Box::clone(&fs_map).iter_mut().rev() {
        let index = match fs_map.iter().position(|block| *block == None) {
            Some(val) => val,
            None => break,
        };
        fs_map[index] = *block;
        fs_map.pop();
   }
    *fs_map
}

fn generate_filesystem(input: String) -> Vec<Option<i64>> {
    let mut fs_map: Vec<Option<i64>> = Vec::new();
    let mut id = 0;
    for (i, c) in input.chars().enumerate() {
        match i % 2 {
            0 => {
                fs_map.append(&mut vec![Some(id); c.to_digit(10).unwrap().try_into().unwrap()]);
                id += 1;
            },
            1 => fs_map.append(&mut vec![None; c.to_digit(10).unwrap().try_into().unwrap()]),
            _ => unreachable!()
        }
    }
    fs_map
}

fn get_checksum(fs_map: Vec<Option<i64>>) -> i64 {
    let mut total: i64 = 0;
    let mut i: usize = 0;
    while i < fs_map.len() && fs_map[i] != None {
        total += i as i64 * fs_map[i].unwrap();
        i += 1;
    }
    total
}

fn process_input() -> String {
    let filename = env::args()
        .last()
        .unwrap();
    read_to_string(filename)
        .expect("File not found")
        .trim()
        .to_string()
}

