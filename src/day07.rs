extern crate regex;
use self::regex::{Regex};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{HashMap, HashSet};

type Deps = HashMap<char, String>;
struct Graph {
    steps: HashSet<char>,
    requires: Deps,
    required: Deps,
}

fn fulfill(required: &mut Deps, requires: &mut Deps, step: char) {
    for c in required.entry(step).or_insert(String::new()).chars() {
        let mut length = None;
        requires
            .entry(c)
            .and_modify(|entry| {
                match entry.find(step) {
                    Some(index) => {
                        entry.remove(index);
                        length = Some(entry.len());
                    },
                    _ => (),
                }
            });

        match length {
            Some(0) => { requires.remove(&c); },
            _ => (),
        }
    }
}

fn part1(graph: &Graph) {
    let mut order = String::with_capacity(26);
    let mut available = Vec::with_capacity(26);
    let mut required = graph.required.clone();
    let mut requires = graph.requires.clone();

    while order.len() < graph.steps.len() {
        available.clear();

        for step in graph.steps.iter() {
            if order.find(*step) == None && !requires.contains_key(&step) {
                available.push(*step);
            }
        }


        available.sort();
        let step = available[0];
        fulfill(&mut required, &mut requires, step);
        
        order.push(step);
    }

    println!("Order: {}", order);
}

fn part2(graph: &Graph) {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut taken = HashSet::new();
    let mut done = String::with_capacity(26);
    let mut available = Vec::with_capacity(26);
    let mut required = graph.required.clone();
    let mut requires = graph.requires.clone();
    let mut workers = 5;
    let mut tasks = Vec::new();
    let mut time = 0;

    while done.len() < graph.steps.len() {
        available.clear();

        for step in graph.steps.iter() {
            if taken.contains(step) {
                continue;
            }
            if done.find(*step) != None {
                continue;
            }
            if requires.contains_key(&step) {
                continue;
            }
            available.push(*step);
        }


        available.sort();

        for step in available.iter().take(workers) {
            let duration = 61 + alphabet.find(*step).unwrap();
            tasks.push((*step, duration));
            taken.insert(*step);
            workers -= 1;
        }

        let mut min = 100;
        for (_, duration) in tasks.iter() {
            if *duration < min {
                min = *duration;
            }
        }

        time += min;
        tasks.retain(|(step, duration)| {
            if *duration > min {
                true
            } else {
                workers += 1;
                done.push(*step);
                fulfill(&mut required, &mut requires, *step);
                false
            }
        });
        tasks = tasks
            .iter()
            .map(|(step, duration)| (*step, duration - min))
            .collect();
    }

    println!("Order: {} ({}s)", done, time);
}

pub fn run() {
    let file =  File::open("./input/day07.txt").expect("File not found");
    let input = BufReader::new(file);
    let mut graph = Graph {
        steps: HashSet::new(),
        requires: HashMap::new(),
        required: HashMap::new(),
    };
    let pattern = Regex::new("^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();

    for l in input.lines() {
        let line = l.unwrap();
        let caps = pattern.captures_iter(&line).next().unwrap();
        let dependency = caps[1].chars().next().unwrap();
        let dependent = caps[2].chars().next().unwrap();

        graph.steps.insert(dependency);
        graph.steps.insert(dependent);

        let entry = graph.requires.entry(dependent).or_insert(String::with_capacity(5));
        entry.push(dependency);

        let entry = graph.required.entry(dependency).or_insert(String::with_capacity(5));
        entry.push(dependent);
    }

    println!("- Part 1 -");
    part1(&graph);
    println!("- Part 2 -");
    part2(&graph);
}
