use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));
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
