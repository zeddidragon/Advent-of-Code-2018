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

    fn to_string(&self) -> String {
        return format!(
            "[{}-{:02}-{:02} {:02}-{:02}] {}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            match self.guard {
                -1 => format!("falls asleep"),
                0 => format!("wakes up"),
                _ => format!("Guard #{} begins shift", self.guard),
            }
        )
    }
}

const TIME : &str = r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})";
const SHIFT : &str = r"Guard #(\d{1,4}) begins shift$";
const SLEEP : &str = "falls asleep";
const WAKE : &str = "wakes up";

fn part1(entries: &Vec<Entry>) {
    let mut slept_at = 0;
    let mut current_guard = 0;
    let mut guard_sleep_times = HashMap::new();
    for entry in entries {
        match entry.guard {
            -1 => {
                slept_at = entry.minute as u32;
            },
            0 => {
                let time = guard_sleep_times.entry(current_guard).or_insert(0);
                *time += entry.minute as u32 - slept_at;
            },
            _ => {
                current_guard = entry.guard;
            }
        }
    }

    let mut sleepiest_guard = 0;
    let mut longest_sleep = 0;
    for (guard, sleep_time) in guard_sleep_times {
        if sleep_time > longest_sleep {
            sleepiest_guard = guard;
            longest_sleep = sleep_time;
        }
    }

    let mut sleep_minutes = [0; 60];

    for entry in entries {
        match entry.guard {
            -1 => {
                slept_at = entry.minute as u32;
            },
            0 => {
                if current_guard != sleepiest_guard {
                    continue;
                }

                for i in slept_at..(entry.minute as u32) {
                    sleep_minutes[i as usize] += 1;
                }
            },
            _ => {
                current_guard = entry.guard;
            }
        }
    }

    let mut sleepiest_minute = 0;
    let mut highest_frequency : u32 = 0;

    for (minute, frequency) in sleep_minutes.iter().enumerate() {
        if *frequency > highest_frequency {
            sleepiest_minute = minute;
            highest_frequency = *frequency;
        }
    }

    println!("Sleepiest guard: #{} ({} minutes)", sleepiest_guard, longest_sleep);
    println!("Sleepiest minute: #{} ({} nights)", sleepiest_minute, highest_frequency);
    println!("Checksum: {}", (sleepiest_guard as usize) * sleepiest_minute);
}

fn part2(shifts: &Vec<Entry>) {
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

pub fn run() {
    let file =  File::open("./input/day04.txt").expect("File not found");
    let input = BufReader::new(file);
    let mut entries = to_entries(input);
    entries.sort_unstable_by(|a, b| a.sortable().cmp(&b.sortable()));

    println!("- Part 1 -");
    part1(&entries);
    println!("- Part 2 -");
    part2(&entries);
}
