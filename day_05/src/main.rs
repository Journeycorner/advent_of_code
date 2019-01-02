use rayon::prelude::*;
use regex::Regex;
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
                Regex::new(&format!(
                    "[{}{}]",
                    remove_letter,
                    remove_letter.to_ascii_uppercase()
                ))
                .unwrap()
                .replace_all(&input_two, "")
            })
            .map(|cleansed| react_recursively(&cleansed))
            .min()
            .unwrap();
        println!("The solution of part 2 is {}", min_length);
    });

    let _ = part_one.join();
    let _ = part_two.join();

    println!("{:#?}", now.elapsed());
}

fn react_recursively(input: &str) -> usize {
    let mut q = Vec::new();

    for c in input.bytes() {
        if q.is_empty() {
            q.push(c);
        } else {
            let last = q.last().unwrap();

            if reacts(*last, c) {
                q.pop();
            } else {
                q.push(c);
            }
        }
    }

    q.len()
}

fn reacts(b1: u8, b2: u8) -> bool {
    if b1 < b2 {
        b2 - b1 == 32
    } else {
        b1 - b2 == 32
    }
}
