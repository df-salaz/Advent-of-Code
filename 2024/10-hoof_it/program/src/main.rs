use std::{collections::HashMap, env, fs};

fn main() {
    let map = parse_input();

    let mut score_1 = 0;
    let mut score_2 = 0;
    for point in &map {
        if *point.1 == 0 {
            let get_score_1 = get_score_1(*point.0, &map);
            score_1 += get_score_1;
            let get_score_2 = get_score_2(*point.0, &map);
            score_2 += get_score_2;
        }
    }

    println!("Part 1:");
    println!("{score_1}");

    println!("Part 2:");
    println!("{score_2}");
}

fn get_score_1(coord: Coord, map: &HashMap<(i32, i32), i32>) -> i32 {
    get_score_1_helper(coord, map, &mut Vec::new())
}

fn get_score_1_helper(coord: Coord, map: &HashMap<Coord, i32>, seen: &mut Vec<Coord>) -> i32 {
    let mut score = 0;

    let neighbors = get_neighbors(coord);
    for new_coord in &neighbors {
        if map
            .get(new_coord)
            .is_some_and(|x| *x == map.get(&coord).unwrap() + 1)
        {
            if *map.get(new_coord).unwrap() == 9 {
                if seen.contains(new_coord) {
                    continue;
                };
                seen.push(*new_coord);
                score += 1;
            } else {
                score += get_score_1_helper(*new_coord, map, seen);
            }
        }
    }

    score
}

fn get_score_2(coord: Coord, map: &HashMap<Coord, i32>) -> i32 {
    let mut score = 0;

    let neighbors = get_neighbors(coord);
    for new_coord in &neighbors {
        if map
            .get(new_coord)
            .is_some_and(|x| *x == map.get(&coord).unwrap() + 1)
        {
            if *map.get(new_coord).unwrap() == 9 {
                score += 1;
            } else {
                score += get_score_2(*new_coord, map);
            }
        }
    }

    score
}

fn get_neighbors(coord: (i32, i32)) -> [(i32, i32); 4] {
    let neighbors: [Coord; 4] = [
        (coord.0 + 1, coord.1),
        (coord.0, coord.1 + 1),
        (coord.0 - 1, coord.1),
        (coord.0, coord.1 - 1),
    ];
    neighbors
}

fn parse_input() -> HashMap<Coord, i32> {
    let mut map = HashMap::new();

    let file_string = fs::read_to_string(env::args().last().unwrap()).expect("Not a file!");

    for (y, line) in file_string.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x.try_into().expect("Line of input too long!");
            let y = y.try_into().expect("Height of input too long!");
            let c = c
                .to_digit(10)
                .expect("Non-digit character in input!")
                .try_into()
                .unwrap();

            map.insert((x, y), c);
        }
    }

    map
}

type Coord = (i32, i32);
