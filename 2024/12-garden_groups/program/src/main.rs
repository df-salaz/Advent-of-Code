use std::{collections::HashMap, env, fs};

fn main() {
    let (id_map, total_ids) = index_input();

    let total = part_1(total_ids, &id_map);
    println!("Part 1: {total}");

    let total = part_2(total_ids, &id_map);
    println!("Part 2: {total}");
}

fn part_1(total_ids: i32, id_map: &HashMap<(i32, i32), i32>) -> i32 {
    let mut total = 0;
    for id in 0..total_ids {
        total += {
            let field: &HashMap<Coord, i32> = id_map;
            let area = get_area(id, field);
            let perimeter = get_perimeter(id, field);
            let result = area * perimeter;

            result
        };
    }
    total
}

fn part_2(total_ids: i32, id_map: &HashMap<(i32, i32), i32>) -> i32 {
    let mut total = 0;
    for id in 0..total_ids {
        total += {
            let field: &HashMap<Coord, i32> = id_map;
            let area = get_area(id, field);
            let sides = get_sides(id, field);
            let result = area * sides;

            result
        };
    }
    total
}

fn index_input() -> (HashMap<(i32, i32), i32>, i32) {
    let input = parse_input();

    let mut ids: HashMap<Coord, i32> = HashMap::with_capacity(input.len());
    let mut id = 0;

    for (coord, flower) in input.iter() {
        let flood_tag_flowers = flood_tag_flowers(&mut ids, &input, coord, flower, &mut id);
        if flood_tag_flowers {
            id += 1;
        }
    }
    (ids, id)
}

fn flood_tag_flowers(
    ids: &mut HashMap<(i32, i32), i32>,
    field: &HashMap<Coord, char>,
    coord: &(i32, i32),
    flower: &char,
    id: &mut i32,
) -> bool {
    if let Some(_) = ids.get(coord) {
        return false;
    }
    ids.insert(*coord, *id);

    let orthogonal: [Coord; 4] = [
        (coord.0 + -1, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0, coord.1 + -1),
        (coord.0, coord.1 + 1),
    ];

    for coord in orthogonal {
        if let Some(neighbor) = field.get(&coord) {
            if *neighbor == *flower {
                flood_tag_flowers(ids, field, &coord, flower, id);
            }
        }
    }

    true
}

fn get_perimeter(id: i32, field: &HashMap<(i32, i32), i32>) -> i32 {
    let mut total = 0;
    for (coord, _) in field.iter().filter(|(_, i)| **i == id) {
        let orthogonal: [Coord; 4] = [
            (coord.0 + -1, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 + -1),
            (coord.0, coord.1 + 1),
        ];

        for test in orthogonal.iter() {
            let found_index = field.get(test);
            match found_index {
                Some(i) if *i == id => {}
                _ => total += 1,
            }
        }
    }
    total
}

fn get_area(id: i32, field: &HashMap<(i32, i32), i32>) -> i32 {
    let count = field
        .iter()
        .filter(|(_, i)| **i == id)
        .count()
        .try_into()
        .unwrap();

    count
}

fn get_sides(id: i32, field: &HashMap<(i32, i32), i32>) -> i32 {
    // Since every corner is associated with two sides, one of which is shared with another corner,
    // "all we need to do" is count corners
    //
    // x marks the spot; We are checking x's corner c
    // these check will be done four times, once for each rotation around c
    //
    // c a
    // b x
    //
    // if a and b are both not x, then this corner is a convex corner.
    // if a and b are both x, but c is not, this corner is a concave corner.
    // do this for every corner of every index
    let mut total = 0;
    for (c, _) in field.iter().filter(|(_, i)| **i == id) {
        let rotations: [(Coord, Coord, Coord); 4] = [
            ((c.0, c.1 + 1), (c.0 - 1, c.1), (c.0 - 1, c.1 + 1)),
            ((c.0, c.1 + 1), (c.0 + 1, c.1), (c.0 + 1, c.1 + 1)),
            ((c.0, c.1 - 1), (c.0 - 1, c.1), (c.0 - 1, c.1 - 1)),
            ((c.0, c.1 - 1), (c.0 + 1, c.1), (c.0 + 1, c.1 - 1)),
        ];

        for (a, b, c) in rotations.iter() {
            let (neighbor_a, neighbor_b, corner_c) = (a, b, c);

            let a = field.get(neighbor_a);
            let b = field.get(neighbor_b);
            let c = field.get(corner_c);

            if a != Some(&id) && b != Some(&id) {
                total += 1;
                continue;
            }
            if a == Some(&id) && b == Some(&id) && c != Some(&id) {
                total += 1;
                continue;
            }
        }
    }
    total
}

type Coord = (i32, i32);

fn parse_input() -> HashMap<Coord, char> {
    let vec_2d: Vec<Vec<char>> = fs::read_to_string(env::args().last().unwrap())
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut map = HashMap::new();
    for (y, line) in vec_2d.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            map.insert((x.try_into().unwrap(), y.try_into().unwrap()), *c);
        }
    }
    map
}
