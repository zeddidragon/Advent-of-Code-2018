extern crate regex;
use self::regex::{Regex};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{HashMap, HashSet};

struct Graph {
    steps: HashSet<char>,
    requires: HashMap<char, String>,
    required: HashMap<char, String>,
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
        
        for c in required.entry(step).or_insert(String::new()).chars() {
            let length;
            {
                let mut entry = requires.entry(c).or_insert(String::new());
                let index = entry.find(step).unwrap();
                entry.remove(index);
                length = entry.len();
            }

            if length == 0 {
                requires.remove(&c);
            }
        }

        order.push(step);
    }

    println!("Order: {}", order);
}

fn part2(graph: &Graph) {
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
