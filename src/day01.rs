use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn part1(changes: &Vec<i32>) {
    let mut x : i32 = 0;
    for &change in changes {
        x += change;
    }
    println!("The frequency is: {}", x)
}

fn part2(changes: &Vec<i32>) {
    let mut x : i32 = 0;
    let mut existing = HashSet::new();
    existing.insert(x);
    loop {
        for &change in changes {
            x += change;
            if existing.contains(&x) {
                println!("Found repeated frequency: {}", x);
                return;
            } else {
                existing.insert(x);
            }
        }
    }
}

pub fn run() {
    let file =  File::open("./input/day01.txt").expect("File not found");
    let input = BufReader::new(file);
    let numbers : Vec<i32> = input
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    println!("- Part 1 -");
    part1(&numbers);
    println!("- Part 2 -");
    part2(&numbers);
}
