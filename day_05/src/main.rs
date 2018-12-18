use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part_one(&input);
}

fn part_one(input: &str) {
    println!("Input size is {}", input.len());
    println!(
        "The solution of part 1 is {}",
        react_recursively(&input).len()
    );
}

fn react_recursively(input: &str) -> String {
    let mut changes = false;
    let mut iterator = input.chars().peekable();
    let mut output = String::new();
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
    String::from(output)
}

fn reaction(a: &char, b: &char) -> bool {
    if *a == *b {
        return false;
    } else if a.eq_ignore_ascii_case(b) {
        return true;
    }
    false
}
