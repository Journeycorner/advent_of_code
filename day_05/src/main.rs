use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    println!("The solution of part 1 is {}", react_recursively(&input));
}

fn part_two(input: &str) {
    let min_length = "abcdefghijklmnopqrstuvwxyz"
        .par_chars()
        .map(|remove_letter| {
            let without_letter = input
                .replace(remove_letter, "")
                .replace(remove_letter.to_ascii_uppercase(), "");
            react_recursively(&without_letter)
        })
        .min()
        .unwrap();
    println!("The solution of part 2 is {}", min_length);
}

fn react_recursively(input: &str) -> usize {
    let mut changes = false;
    let mut iterator = input.chars().peekable();
    let mut output = String::with_capacity(input.len());
    loop {
        if let Some(current) = iterator.next() {
            if let Some(next) = iterator.peek() {
                if !reaction(&current, next) {
                    output.push(current);
                } else {
                    // skip both current and next item
                    iterator.next();
                    changes = true;
                }
            } else if changes {
                output.push(current);
                return react_recursively(&output);
            } else {
                output.push(current);
                break;
            }
        } else {
            break;
        }
    }
    output.len()
}

fn reaction(a: &char, b: &char) -> bool {
    if *a != *b && a.eq_ignore_ascii_case(b) {
        return true;
    }
    false
}
