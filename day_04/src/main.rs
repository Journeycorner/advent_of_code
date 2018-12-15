use chrono::*;
use lazy_static::*;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() {
    let mut input: Vec<ShiftEntry> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    input.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    for i in input {
        println!("{:?}", i);
    }
    // println!("The result of part one is {}.", solutions.0);
    // println!("The result of part two is {}.", solutions.1);
}

#[derive(Debug)]
struct ShiftEntry {
    timestamp: NaiveDateTime,
    action: Action,
}

impl FromStr for ShiftEntry {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}").unwrap();
            static ref GUARD_ID_REGEX: Regex = Regex::new(r"#(\d{1,4})").unwrap();
        }
        let raw_date_time = REGEX.captures(s).unwrap().get(0).unwrap().as_str();
        let action;
        if s.contains("wakes up") {
            action = Action::WakesUp;
        } else if s.contains("falls asleep") {
            action = Action::FallsAsleep;
        } else {
            let id = GUARD_ID_REGEX
                .captures(s)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            action = Action::BeginsShift(id);
        }

        Ok(ShiftEntry {
            timestamp: NaiveDateTime::parse_from_str(raw_date_time, "%Y-%m-%d %H:%M").unwrap(),
            action: action,
        })
    }
}

#[derive(Debug)]
enum Action {
    BeginsShift(i32),
    WakesUp,
    FallsAsleep,
}
