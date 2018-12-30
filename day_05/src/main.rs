use rayon::prelude::*;
use std::fs;
use std::thread;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();
    let input_two = input.clone();

    let part_one = thread::spawn(move || {
        println!("The solution of part 1 is {}", react_recursively(&input));
    });

    let part_two = thread::spawn(move || {
        let min_length = "abcdefghijklmnopqrstuvwxyz"
            .par_chars()
            .map(|remove_letter| {
                react_recursively(
                    &input_two
                        .replace(remove_letter, "")
                        .replace(remove_letter.to_ascii_uppercase(), ""),
                )
            })
            .min()
            .unwrap();
        println!("The solution of part 2 is {}", min_length);
    });

    let _ = part_one.join();
    let _ = part_two.join();

    println!("{:#?}", now.elapsed());
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
