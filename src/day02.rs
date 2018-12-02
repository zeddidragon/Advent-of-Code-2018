use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn part1(ids: &Vec<Vec<char>>) {
    let mut counter = HashMap::new();
    let mut pairs = 0;
    let mut triplets = 0;

    for id in ids {
        counter.clear();
        let mut id_pairs = 0;
        let mut id_triplets = 0;

        for c in id {
            let entry = counter.entry(c).or_insert(0);
            *entry += 1;
            if *entry == 2 {
                id_pairs += 1;
            } else if *entry == 3 {
                id_pairs -= 1;
                id_triplets += 1;
            } else if *entry == 4 {
                id_triplets -= 1;
            }
        }

        if id_pairs > 0 {
            pairs += 1;
        }
        if id_triplets > 0 {
            triplets += 1;
        }
    }

    println!("Checksum: {}", pairs * triplets);
}

fn part2(ids: &Vec<Vec<char>>) {
    for (j, a) in ids.iter().enumerate() {
        for b in ids.iter().skip(j + 1) {
            let mut diff = 0;
            for (i, c) in a.iter().enumerate() {
                if *c != b[i] {
                    diff += 1;
                }
            }
            if diff == 1 {
                println!("Pair found!");
                for (i, c) in a.iter().enumerate() {
                    if *c == b[i] {
                        print!("{}", c);
                    }
                }
                println!("");
                return ();
            }
        }
    }
}

pub fn run() {
    let file =  File::open("./input/day02.txt").expect("File not found");
    let input = BufReader::new(file);
    let lines : Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    println!("- Part 1 -");
    part1(&lines);
    println!("- Part 2 -");
    part2(&lines);
}
