use chrono::*;
use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() {
    let mut entries: Vec<ShiftEntry> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    println!("The result of part one is {}.", part_one(&entries));
    println!("The result of part two is {}.", part_two(&entries));
}

fn part_one(entries: &Vec<ShiftEntry>) -> u32 {
    let guard_sleep = compute_sleep_minutes(entries);

    let mut max_guard_id = 0;
    let mut max_minutes_asleep = 0;

    for (guard_id, value) in &guard_sleep {
        let total_minutes_asleep: u32 = value.iter().map(|(_, value)| value).sum();
        if total_minutes_asleep > max_minutes_asleep {
            max_minutes_asleep = total_minutes_asleep;
            max_guard_id = *guard_id;
        }
    }

    let mut max_minute_key = 0;
    let mut max_minute_value = 0;

    for (minute, value) in guard_sleep.get(&max_guard_id).unwrap().iter() {
        if *value > max_minute_value {
            max_minute_value = *value;
            max_minute_key = *minute;
        }
    }

    max_guard_id * max_minute_key
}

fn part_two(entries: &Vec<ShiftEntry>) -> u32 {
    let guard_sleep = compute_sleep_minutes(entries);

    let mut max_guard_id = 0;
    let mut max_minutes_asleep = 0;
    let mut max_minute_asleep = 0;

    for (guard_id, value) in &guard_sleep {
        for (minute, count) in value.iter() {
            if *count > max_minutes_asleep {
                max_minutes_asleep = *count;
                max_guard_id = *guard_id;
                max_minute_asleep = *minute;
            }
        }
    }

    println!("{}:{}", max_guard_id, max_minute_asleep);
    max_guard_id * max_minute_asleep
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
    BeginsShift(u32),
    WakesUp,
    FallsAsleep,
}

fn compute_sleep_minutes(entries: &Vec<ShiftEntry>) -> HashMap<u32, HashMap<u32, u32>> {
    let mut current_guard_id: u32 = 0;
    let mut guard_sleep: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let mut entries = entries.iter().peekable();

    loop {
        if let Some(entry) = entries.next() {
            // set guard id and skip on shift start
            if let Action::BeginsShift(id) = entry.action {
                current_guard_id = id;
                continue;
            }
            // skip wake up
            else if let Action::WakesUp = entry.action {
                continue;
            }

            // skip entries that do not belong to shift
            let within_shift = entry.timestamp.time().hour() == 0;
            if !within_shift {
                continue;
            }

            // increase sleep minutes from this minute till next entry minute
            let from_minute = entry.timestamp.minute();
            let to_minute = entries.peek().unwrap().timestamp.minute();
            for minute in from_minute..=to_minute {
                let guard = guard_sleep.get_mut(&current_guard_id);
                if guard.is_none() {
                    let mut minute_map: HashMap<u32, u32> = HashMap::new();
                    minute_map.insert(minute, 1);
                    guard_sleep.insert(current_guard_id, minute_map);
                } else {
                    let guard = guard.unwrap();
                    let entry = guard.get_mut(&minute);
                    if entry.is_none() {
                        guard.insert(minute, 1);
                    } else {
                        *entry.unwrap() += 1;
                    }
                }
            }
        } else {
            break;
        }
    }
    guard_sleep
}
