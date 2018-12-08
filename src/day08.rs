use std::io::{BufReader, BufRead};
use std::fs::File;

struct Node {
    child_count: u8,
    meta_count: u8,
    children: Vec<Node>,
    meta: Vec<u8>,
}

impl Node {
    fn new() -> Node {
        Node {
            child_count: 0,
            meta_count: 0,
            children: Vec::with_capacity(0),
            meta: Vec::with_capacity(0),
        }
    }

    fn reserve(&mut self) {
        self.children.reserve_exact(self.child_count as usize);
        self.meta.reserve_exact(self.meta_count as usize);
    }

    #[inline]
    fn meta_sum(&self) -> usize {
        let mut sum = 0;

        for byte in self.meta.iter() {
            sum += *byte as usize;
        }

        sum
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;

        for child in self.children.iter() {
            sum += child.checksum();
        }

        self.meta_sum() + sum
    }

    fn value(&self) -> usize {
        if self.child_count < 1 {
            return self.meta_sum();
        }

        let mut sum = 0;

        for byte in self.meta.iter() {
            if *byte < 1 {
                continue;
            }

            match self.children.get((*byte as usize) - 1) {
                Some(child) => sum += child.value(),
                None => (),
            }
        }

        sum
    }
}

#[derive(Clone, Copy)]
enum Mode {
    HeaderChild,
    HeaderMeta,
    Meta,
}

fn part1(root: &Node) {
    println!("Checksum: {}", root.checksum());
}

fn part2(root: &Node) {
    println!("Value: {}", root.value());
}

pub fn run() {
    let file =  File::open("./input/day08.txt").expect("File not found");
    let input = BufReader::new(file);
    let bytes : Vec<u8> = input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut stack = Vec::new();
    let mut mode = Mode::HeaderChild;
    let mut current = Node::new();

    for byte in bytes {
        match mode {
            Mode::HeaderChild => {
                current.child_count = byte;
                mode = Mode::HeaderMeta;
            }

            Mode::HeaderMeta => {
                current.meta_count = byte;
                current.reserve();
                if current.child_count > 0 {
                    let node = Node::new();
                    stack.push(current);
                    current = node;
                    mode = Mode::HeaderChild;
                } else {
                    mode = Mode::Meta;
                }
            }

            Mode::Meta => {
                current.meta.push(byte);

                if current.meta.len() < (current.meta_count as usize) {
                    continue;
                }
                if stack.is_empty() {
                    break;
                }

                let old = current;
                current = stack.pop().unwrap();
                current.children.push(old);
                if current.children.len() < (current.child_count as usize) {
                    let node = Node::new();
                    stack.push(current);
                    current = node;
                    mode = Mode::HeaderChild;
                } else {
                    mode = Mode::Meta;
                }
            }
        }
    }

    println!("- Part 1 -");
    part1(&current);
    println!("- Part 2 -");
    part2(&current);
}
