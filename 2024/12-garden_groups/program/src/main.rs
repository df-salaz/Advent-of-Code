use std::{collections::HashMap, env, fs};

fn main() {
    let (id_map, total_ids) = parse_input();

    let price = part_1(total_ids, &id_map);
    println!("Part 1: {price}");

    let price = part_2(total_ids, &id_map);
    println!("Part 2: {price}");
}

fn part_1(total_ids: i32, field: &HashMap<(i32, i32), i32>) -> i32 {
    let mut price = 0;
    for id in 0..total_ids {
        price += {
            let area = field.iter().filter(|(_, i)| **i == id).count() as i32;
            let perimeter = get_perimeter(id, field);

            area * perimeter
        };
    }
    price
}

fn part_2(total_ids: i32, field: &HashMap<(i32, i32), i32>) -> i32 {
    let mut price = 0;
    for id in 0..total_ids {
        price += {
            let area = field.iter().filter(|(_, i)| **i == id).count() as i32;
            let sides = get_sides(id, field);

            area * sides
        };
    }
    price
}

type Coord = (i32, i32);

fn get_perimeter(id: i32, field: &HashMap<(i32, i32), i32>) -> i32 {
    let mut total = 0;
    for (coord, _) in field.iter().filter(|(_, i)| **i == id) {
        let orthogonal: [Coord; 4] = [
            (coord.0 + -1, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 + -1),
            (coord.0, coord.1 + 1),
        ];

        for coord in orthogonal.iter() {
            let neighbor = field.get(coord);
            match neighbor {
                Some(i) if *i == id => {}
                // increment our perimeter total by 1 for each dissimilar orthogonal neighbor
                _ => total += 1,
            }
        }
    }
    total
}

/// Counts the number of sides to a flower bed
fn get_sides(id: i32, field: &HashMap<(i32, i32), i32>) -> i32 {
    // x represents the current point in:
    //      c a
    //      b x
    // If a and b are both not x, then this corner is a convex corner.
    // If a and b are both x, but c is not, this corner is a concave corner.
    // Rotate 4 times per point and accumulate
    // Rust's option type makes bounds checking unnecessary and I cummed at that fact
    let mut sides = 0;
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
                sides += 1;
                continue;
            }
            if a == Some(&id) && b == Some(&id) && c != Some(&id) {
                sides += 1;
                continue;
            }
        }
    }
    sides
}

/// Returns a hash map of every distinct flower bed, indexed with a unique id
fn parse_input() -> (HashMap<(i32, i32), i32>, i32) {
    let input = {
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
    };

    let mut ids: HashMap<Coord, i32> = HashMap::with_capacity(input.len());
    let mut id = 0;

    for (coord, flower) in input.iter() {
        let flood_tag_flowers = flood_fill_flower_id(&mut ids, &input, coord, flower, &mut id);
        if flood_tag_flowers {
            id += 1;
        }
    }
    (ids, id)
}

/// Returns true if region filled successfully, false if region already occupied
fn flood_fill_flower_id(
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
            if neighbor == flower {
                flood_fill_flower_id(ids, field, &coord, flower, id);
            }
        }
    }

    true
}
