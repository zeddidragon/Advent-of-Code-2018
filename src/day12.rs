use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{HashSet};
use std::cmp::{min, max};

fn hash(slice: &[bool], offset: i64) -> i64 {
    let mut sum : i64 = 0;
    for (i, plant) in slice.iter().enumerate() {
        if *plant {
            sum += 2_i64.pow(((i as i64) - offset) as u32);
        }
    }
    sum
}

fn sum(slice: &[bool], offset: i64) -> i64 {
    let mut sum = 0;
    for (i, plant) in slice.iter().enumerate() {
        if *plant {
            sum += (i as i64) + offset;
        }
    }
    sum
}

fn mutate(map: &Vec<bool>, pots: &mut Vec<bool>, next: &mut Vec<bool>, left: &mut i64, right: &mut i64) {
    next.clear();
    let size = pots.len();
    let from = *left - 1;
    let to = *left + (size as i64) + 2;
    let mut next_left = *left;
    for (i, index) in (from..to).enumerate() {
        let l = max((i as i64) - 3, 0) as usize;
        let r = min((i as i64) + 2, size as i64) as usize;
        let offset = min(0, index - *left - 2) as i64;
        let map_index = hash(&pots[l..r], offset) as usize;
        let plant = map.get(map_index).unwrap();

        if *plant && index < next_left {
            next_left = index;
        } else if !*plant && (index < next_left || index > *right) {
            continue;
        } else if index > *right {
            *right = index;
        }
        next.push(*plant);
    }

    *left = next_left;
}

fn part1(map: &Vec<bool>, initial: &Vec<bool>) {
    let mut pots = initial.clone();
    let mut next = Vec::with_capacity(initial.len() + 40);
    let mut left : i64 = 0;
    let mut right : i64 = (pots.len() - 1) as i64;

    for _ in 0..20 {
        mutate(map, &mut pots, &mut next, &mut left, &mut right);

        let tmp = pots;
        pots = next;
        next = tmp;
    }

    println!("Sum is {}", sum(&pots, left));
}

fn state(pots: &[bool]) -> String {
    let mut ret = String::with_capacity(pots.len());
    let mut empties = 0;
    let mut l : Option<u64> = None;
    for (i, plant) in pots.iter().enumerate() {
        if *plant && l == None {
            l = Some(i as u64);
            empties = 0;

        } else if *plant {
            for _ in 0..empties {
                ret.push('.');
            }
            ret.push('#');
            empties = 0;

        } else {
            empties += 1;
        }
    }
    ret
}

fn part2(map: &Vec<bool>, initial: &Vec<bool>) {
    let mut seen : HashSet<String> = HashSet::new();
    let mut pots = initial.clone();
    let mut next = Vec::with_capacity(initial.len() + 40);
    let mut left : i64 = 0;
    let mut right : i64 = (pots.len() - 1) as i64;
    let mut i = 0;

    loop {
        let hash = state(&pots);
        if seen.contains(&hash) {
            break;
        }

        seen.insert(hash);
        i += 1;
        mutate(map, &mut pots, &mut next, &mut left, &mut right);
        let tmp = pots;
        pots = next;
        next = tmp;
    }

    // In my case, pattern completely stabilized after exactly 100 cycles
    // The same pattern merely "moves" to the right.
    left += 50_000_000_000 - (i as i64);
 
    println!("Sum is {}", sum(&pots, left));
}

pub fn run() {
    let file =  File::open("./input/day12.txt").expect("File not found");
    let input = BufReader::new(file);
    let mut lines = input.lines().map(|l| l.unwrap());
    let initial : Vec<bool> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect();
    let mut map = vec![false; 32]; // 2 ^ 5 different states
    for line in lines.skip(1) {
        let mut split = line.split(" => ");
        let states : Vec<bool> = split
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();
        let in_state = hash(&states, 0);
        let out_state = split.next().unwrap() == "#";
        map[in_state as usize] = out_state;
    }

    println!("- Part 1 -");
    part1(&map, &initial);
    println!("- Part 2 -");
    part2(&map, &initial);
}
