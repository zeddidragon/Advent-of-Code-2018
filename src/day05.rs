extern crate regex;
use self::regex::{Regex};
use std::io::{BufReader, BufRead};
use std::fs::File;

fn collapse(polymer: &str) -> usize {
    let mut chars : Vec<char> = polymer.chars().collect();
    let mut size = chars.len();
    let end = size - 1;
    let mut a = 0;
    let mut b = 1;

    'outer: loop {
        if b > end {
            break;
        }

        let ca = chars[a];
        let cb = chars[b];

        let is_pair : bool =
            if ca.is_ascii_lowercase() {
                cb.is_ascii_uppercase() && ca == cb.to_ascii_lowercase()
            } else {
                cb.is_ascii_lowercase() && ca == cb.to_ascii_uppercase()
            };

        if !is_pair {
            a = b;
            b += 1;
            continue;
        }

        size -= 2; // Two units reacted and got removed
        chars[a] = ' ';
        chars[b] = ' ';

        while chars[a] == ' ' { // Seek back to unremoved unit
            if a == 0 { // If we hit the beginning, skip forward instead
                a = b + 1;
                b += 2;
                continue 'outer;
            }
            a -= 1;
        }
        b += 1;
    }

    return size;
}

fn part1(polymer: &str) {
    println!("\rAfter reactions, {} units remain", collapse(polymer));
}

fn part2(polymer: &str) {
    let mut most_blocking = 'a';
    let mut lowest = polymer.len();
    for c in "abcdefghijklmopqrstuvwxyz".chars() {
        let pattern = Regex::new(&format!("{}|{}", c, c.to_ascii_uppercase()));
        let result = collapse(&pattern.unwrap().replace_all(polymer, ""));
        if result < lowest {
            lowest = result;
            most_blocking = c;
        }
    }

    println!(
        "Most blocking type: {} ({} units after removing)",
        most_blocking,
        lowest,
    );
}

pub fn run() {
    let file =  File::open("./input/day05.txt").expect("File not found");
    let polymer = BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap();

    println!("- Part 1 -");
    part1(&polymer);
    println!("- Part 2 -");
    part2(&polymer);
}
