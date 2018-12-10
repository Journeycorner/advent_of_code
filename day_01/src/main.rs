use std::collections::HashSet;
use std::fs;

fn main() {
    // part one
    let result: i32 = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|slice| slice.parse::<i32>().unwrap())
        .sum();

    println!("The result of part one is {}.", result);

    // part two
    let mut previous_sums = vec![0];
    let mut keys = HashSet::new();
    keys.insert(0);

    fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|slice| slice.parse::<i32>().unwrap())
        .map(|frequency| {
            let sum = previous_sums.last().unwrap() + frequency;
            previous_sums.push(sum);
            sum
        })
        .take_while(|sum| !keys.insert(*sum))
        .for_each(|_| {});

    println!(
        "The result of part two is {}.",
        previous_sums.last().unwrap()
    );
}
