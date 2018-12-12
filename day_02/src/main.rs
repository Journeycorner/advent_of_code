use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));
}

fn part_one(input: &str) -> i32 {
    let mut exactly_two_double_matches = 0;
    let mut exactly_three_matches = 0;

    for line in input.lines() {
        let mut letter_map: HashMap<&char, usize> = HashMap::new();

        for character in line.chars() {
            if letter_map.contains_key(&character) {
                if *letter_map.get(&character).unwrap() == 3 {
                    // dont need four matches
                    continue;
                }
                *letter_map.get_mut(&character).unwrap() += 1;
            }
        }

        let mut double_matches = 0;
        let mut triple_matches = 0;
        letter_map
            .values()
            .filter(|value| **value >= 2)
            .for_each(|value| {
                if *value == 2 {
                    double_matches += 1;
                } else if *value == 3 {
                    triple_matches += 1;
                }
            });
        if double_matches == 2 {
            exactly_two_double_matches += 1;
        }
        if triple_matches == 1 {
            exactly_three_matches += 1;
        }
    }

    exactly_two_double_matches * exactly_three_matches
}
