use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{HashMap};

fn part1(grid: &Vec<Vec<char>>, mut carts: &mut Vec<Cart>) {
    loop {
        match step(&grid, &mut carts) {
            Some((x, y)) => {
                println!("First collision at: ({},{})", x, y);
                break;
            },
            None => {}
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq)]
struct Cart {
    id: usize,
    x: usize,
    y: usize,
    dir: char,
    i: i8,
}

impl Cart {
    fn step(&mut self) {
        match &self.dir {
            '>' => { self.x += 1 },
            '<' => { self.x -= 1 },
            'v' => { self.y += 1 },
            '^' => { self.y -= 1 },
            _ => {},
        }
    }

    fn turn(&mut self, track: char) {
        match (&self.dir, track) {
            ('<', '/') =>  { self.dir = 'v' },
            ('>', '/') =>  { self.dir = '^' },
            ('v', '/') =>  { self.dir = '<' },
            ('^', '/') =>  { self.dir = '>' },
            ('<', '\\') =>  { self.dir = '^' },
            ('>', '\\') =>  { self.dir = 'v' },
            ('v', '\\') =>  { self.dir = '>' },
            ('^', '\\') =>  { self.dir = '<' },
            (_, '+') => {
                self.i = (self.i + 1) % 3;
                match (&self.dir, &self.i) {
                    ('<', 1) =>  { self.dir = 'v' },
                    ('>', 1) =>  { self.dir = '^' },
                    ('v', 1) =>  { self.dir = '>' },
                    ('^', 1) =>  { self.dir = '<' },
                    ('<', 0) =>  { self.dir = '^' },
                    ('>', 0) =>  { self.dir = 'v' },
                    ('v', 0) =>  { self.dir = '<' },
                    ('^', 0) =>  { self.dir = '>' },
                    _ => {},
                }
            }
            (_, _) => { },
        }
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> std::cmp::Ordering {
        use std::cmp::Ordering::{Less, Greater, Equal};
        match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
            (_, Less) => Less,
            (_, Greater) => Greater,
            (Less, _) => Less,
            (Greater, _) => Greater,
            _ => Equal,
        }
    }
}

#[allow(dead_code)]
fn draw(grid: &Vec<Vec<char>>, carts: &Vec<Cart>) {
    let mut xy_carts = HashMap::new();
    for cart in carts.iter() {
        xy_carts.insert((cart.x, cart.y), cart.dir);
    }

    for (j, line) in grid.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            if xy_carts.contains_key(&(i, j)) {
                print!("{}", xy_carts.get(&(i, j)).unwrap());
            } else {
                print!("{}", c);
            }
        }
        println!("");
    }
}

fn step(grid: &Vec<Vec<char>>, carts: &mut Vec<Cart>)
        -> Option<(usize, usize)> {
    let mut xy_carts = HashMap::new();
    for cart in carts.iter() {
        xy_carts.insert((cart.x, cart.y), cart.dir);
    }

    carts.sort();

    for cart in carts.iter_mut() {
        let prev = (cart.x, cart.y);
        &cart.step();
        let pos = (cart.x, cart.y);
        if xy_carts.contains_key(&pos) {
            return Some((cart.x, cart.y));
        }

        xy_carts.remove(&prev);
        xy_carts.insert(pos, cart.dir);

        let c : char = grid[cart.y][cart.x];
        &cart.turn(c);
    }

    None
}

pub fn run() {
    let file =  File::open("./input/day13.txt").expect("File not found");
    let input = BufReader::new(file);
    let mut carts = vec!();
    let mut grid = vec!();

    for (j, line) in input.lines().enumerate() {
        let mut tracks = vec!();
        for (i, c) in line.unwrap().chars().enumerate() {
            let cart = Cart{id: carts.len(), x: i, y: j, dir: c, i: 0 };
            match c {
                '>' => {
                    carts.push(cart);
                    tracks.push('-');
                },
                'v' => {
                    carts.push(cart);
                    tracks.push('|');
                },
                '<' => {
                    carts.push(cart);
                    tracks.push('-');
                },
                '^' => {
                    carts.push(cart);
                    tracks.push('|');
                },
                _ => {
                    tracks.push(c);
                }
            }
        }
        grid.push(tracks);
    }

    println!("- Part 1 -");
    part1(&grid, &mut carts);
    println!("- Part 2 -");
}
