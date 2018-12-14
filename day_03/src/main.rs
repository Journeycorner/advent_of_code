use std::collections::HashMap;
use std::fs;

const MAX_FIELD_SIZE: i32 = 1000;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));
}

fn part_one(input: &str) -> usize {
    let mut field: HashMap<i32, i8> = HashMap::new();

    input.lines().map(|line| parse(line)).for_each(|claim| {
        for j in claim.y..(claim.y + claim.tall) {
            for i in claim.x..(claim.x + claim.wide) {
                let index = j * MAX_FIELD_SIZE + i;
                let item = field.get_mut(&index);
                if item.is_none() {
                    field.insert(index, 1);
                } else {
                    *item.unwrap() += 1;
                }
            }
        }
    });

    field
        .iter()
        .map(|(_, value)| value)
        .filter(|value| **value >= 2)
        .count()
}

fn parse(line: &str) -> Claim {
    let mut iter = line.split_whitespace().skip(2);

    let x_y_iter = iter.next().unwrap().replace(":", "");
    let mut x_y_iter = x_y_iter.split(",");
    let x = x_y_iter.next().unwrap().parse().unwrap();
    let y = x_y_iter.next().unwrap().parse().unwrap();

    let iter_rect = iter.next().unwrap();
    let mut iter_rect = iter_rect.split("x");
    let wide = iter_rect.next().unwrap().parse().unwrap();
    let tall = iter_rect.next().unwrap().parse().unwrap();

    Claim { x, y, wide, tall }
}

struct Claim {
    x: i32,
    y: i32,
    wide: i32,
    tall: i32,
}
