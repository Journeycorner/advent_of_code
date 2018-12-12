use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));

    println!("The result of part one is {}.", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|slice| slice.parse::<i32>().unwrap())
        .sum()
}

fn part_two(input: &str) -> i32 {
    let mut previous_sum = 0;
    let mut seen = HashSet::new();
    seen.insert(previous_sum);

    loop {
        for line in input.lines() {
            let change: i32 = line.parse().unwrap();
            previous_sum += change;
            if seen.contains(&previous_sum) {
                return previous_sum;
            }
            seen.insert(previous_sum);
        }
    }
}
