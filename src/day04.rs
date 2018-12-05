extern crate regex;
use self::regex::{Regex};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

struct Entry {
    guard: i16,
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl Entry {
    fn sortable(&self) -> u64 {
        return
            (self.minute as u64) +
            (self.hour as u64) * 60 +
            (self.day as u64) * 60 * 24 +
            (self.month as u64) * 60 * 24 * 31 +
            (self.year as u64 - 1500) * 60 * 24 * 31 * 366;
    }
}

const TIME : &str = r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})";
const SHIFT : &str = r"Guard #(\d{1,4}) begins shift$";
const SLEEP : &str = "falls asleep";
const WAKE : &str = "wakes up";

fn part1(guard_minutes: &HashMap<i16, [i16; 60]>) {
    let mut sleepiest_guard = 0;
    let mut sleep_record = 0;

    for (guard, minutes) in guard_minutes {
        let mut total = 0;
        for minute in minutes.iter() {
            total += minute;
        }
        if total > sleep_record {
            sleepiest_guard = *guard;
            sleep_record = total;
        }
    }

    let mut sleepiest_minute = 0;
    let mut frequency_record = 0;
    for (minute, frequency) in guard_minutes[&sleepiest_guard].iter().enumerate() {
        if *frequency > frequency_record {
            sleepiest_minute = minute;
            frequency_record = *frequency;
        }
    }

    println!("Sleepiest guard: #{} ({} minutes)", sleepiest_guard, sleep_record);
    println!("Sleepiest minute: {}m ({} nights)", sleepiest_minute, frequency_record);
    println!("Checksum: {}", (sleepiest_guard as usize) * sleepiest_minute);
}

fn part2(guard_minutes: &HashMap<i16, [i16; 60]>) {
    let mut most_habitual = 0;
    let mut habit_record = 0;
    let mut habit_minute = 0;

    for (guard, minutes) in guard_minutes {
        for (minute, frequency) in minutes.iter().enumerate() {
            if *frequency > habit_record {
                most_habitual = *guard;
                habit_record = *frequency;
                habit_minute = minute;
            }
        }
    }

    println!("Most habitual: #{} ({}m, {} nights)", most_habitual, habit_minute, habit_record);
    println!("Checksum: {}", (most_habitual as usize) * habit_minute);
}

fn to_entries(input: BufReader<File>) -> Vec<Entry> {
    let time_re : Regex = Regex::new(TIME).unwrap();
    let shift_re : Regex = Regex::new(SHIFT).unwrap();

    return input.lines().map(|l| {
        let line = &l.unwrap();
        let guard;

        if line.ends_with(SLEEP) {
            guard = -1
        } else if line.ends_with(WAKE) {
            guard = 0;
        } else {
            guard = shift_re
                .captures_iter(line)
                .next()
                .unwrap()[1]
                .parse()
                .unwrap();
        }

        let caps = time_re.captures_iter(line).next().unwrap();
        return Entry {
            guard: guard,
            year: caps[1].parse().unwrap(),
            month: caps[2].parse().unwrap(),
            day: caps[3].parse().unwrap(),
            hour: caps[4].parse().unwrap(),
            minute: caps[5].parse().unwrap(),
        };
    })
        .collect();
}

fn to_sleep_minutes(entries: Vec<Entry>) -> HashMap<i16, [i16; 60]> {
    let mut slept_at = 0;
    let mut current_guard = 0;
    let mut sleep_minutes = HashMap::new();
    for entry in entries {
        match entry.guard {
            -1 => {
                slept_at = entry.minute as u32;
            },
            0 => {
                let minutes = sleep_minutes.entry(current_guard).or_insert([0; 60]);
                for i in slept_at..(entry.minute as u32) {
                    minutes[i as usize] += 1;
                }
            },
            _ => {
                current_guard = entry.guard;
            }
        }
    }

    return sleep_minutes;
}

pub fn run() {
    let file =  File::open("./input/day04.txt").expect("File not found");
    let input = BufReader::new(file);
    let mut entries = to_entries(input);
    entries.sort_unstable_by(|a, b| a.sortable().cmp(&b.sortable()));

    let guard_minutes = to_sleep_minutes(entries);

    println!("- Part 1 -");
    part1(&guard_minutes);
    println!("- Part 2 -");
    part2(&guard_minutes);
}
