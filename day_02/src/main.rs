use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));
    println!("The result of part two is {}.", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut doubles = 0;
    let mut triples = 0;

    for line in input.lines() {
        let mut letter_map: HashMap<char, usize> = HashMap::new();
        let mut line_doubles = false;
        let mut line_triples = false;

        for character in line.chars() {
            if letter_map.contains_key(&character) {
                let count = letter_map.get(&character).unwrap() + 1;
                if !line_doubles && count == 2 {
                    line_doubles = true;
                } else if !line_triples && count == 3 {
                    line_triples = true;
                }
                if line_doubles && line_triples {
                    break;
                }
                *letter_map.get_mut(&character).unwrap() = count;
            } else {
                letter_map.insert(character.clone(), 1);
            }
        }

        if line_doubles {
            doubles += 1;
        }
        if line_triples {
            triples += 1;
        }
    }

    doubles * triples
}

fn part_two(input: &str) -> String {
    // match every line with each other (inluding itself)
    for a in input.lines() {
        for b in input.lines() {
            let mut result: Vec<char> = Vec::with_capacity(b.len() - 1);
            let mut mismatch = 0;
            let mut a_iterator = a.chars();
            let mut b_iterator = b.chars();
            loop {
                let x = a_iterator.next();
                if x.is_none() {
                    // eol
                    break;
                }
                let x = x.unwrap();
                let y = b_iterator.next().unwrap();

                if x != y {
                    if mismatch == 1 {
                        break;
                    }
                    mismatch += 1;
                    continue;
                }
                result.push(y);
            }
            if result.len() == b.len() - 1 {
                // result has correct length - success!
                return result.iter().collect();
            }
        }
    }
    String::new()
}
