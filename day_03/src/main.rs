use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));
}

fn part_one(input: &str) -> i32 {
    for line in input.lines() {
        let claim = parse_claim(line);
        println!("Successfully parsed input {:?}", claim);
        // TODO fill ...
    }

    0
}

fn parse_claim(line: &str) -> Claim {
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

#[derive(Debug)]
struct Claim {
    x: i32,
    y: i32,
    wide: i32,
    tall: i32,
}
