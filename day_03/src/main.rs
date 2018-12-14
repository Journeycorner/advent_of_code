use std::collections::HashMap;
use std::fs;

const MAX_FIELD_SIZE: i32 = 1000;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let solutions = run(&input);
    println!("The result of part one is {}.", solutions.0);
    println!("The result of part two is {}.", solutions.1);
}

fn run(input: &str) -> (usize, String) {
    let mut non_overlapping_id = String::new();
    let mut field: HashMap<i32, i8> = HashMap::new();
    let claims = input.lines().map(|line| parse(line));
    for claim in claims.clone() {
        for index in claim.range {
            let item = field.get_mut(&index);
            if item.is_none() {
                field.insert(index, 1);
            } else {
                *item.unwrap() += 1;
            }
        }
    }

    for claim in claims {
        if claim.range.iter().any(|i| *field.get(i).unwrap() > 1) {
            continue;
        }
        non_overlapping_id = claim.id;
    }

    (
        field.iter().filter(|(_, value)| **value >= 2).count(),
        non_overlapping_id,
    )
}

fn parse(line: &str) -> Claim {
    let mut iter = line.split_whitespace();
    // id
    let id = iter.next().unwrap().to_string().replace("#", "");
    let mut iter = iter.skip(1);

    let x_y_iter = iter.next().unwrap().replace(":", "");
    let mut x_y_iter = x_y_iter.split(",");
    // x
    let x: i32 = x_y_iter.next().unwrap().parse().unwrap();
    // y
    let y: i32 = x_y_iter.next().unwrap().parse().unwrap();

    let iter_rect = iter.next().unwrap();
    let mut iter_rect = iter_rect.split("x");
    // wide
    let wide: i32 = iter_rect.next().unwrap().parse().unwrap();
    // tall
    let tall: i32 = iter_rect.next().unwrap().parse().unwrap();

    let mut range = Vec::new();
    for j in y..(y + tall) {
        for i in x..(x + wide) {
            let index = j * MAX_FIELD_SIZE + i;
            range.push(index);
        }
    }
    Claim { id, range }
}

struct Claim {
    id: String,
    range: Vec<i32>,
}
