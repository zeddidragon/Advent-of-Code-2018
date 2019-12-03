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

fn part2(grid: &Vec<Vec<char>>, mut carts: &mut Vec<Cart>) {
    while carts.len() > 2 {
        step(&grid, &mut carts);
    }
    println!("Final cart ends up at: ({}, {})", carts[0].x, carts[0].y);
}

#[derive(PartialEq, PartialOrd, Eq)]
struct Cart {
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

    fn collides(&self, other: &Cart) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn turn(&mut self, track: char) {
        if track == '+' { self.i = (self.i + 1) % 3 }
        self.dir = match (&self.dir, track, self.i) {
            ('<', '/', _) =>  'v',
            ('>', '/', _) =>  '^',
            ('v', '/', _) =>  '<',
            ('^', '/', _) =>  '>',
            ('<', '\\', _) => '^',
            ('>', '\\', _) => 'v',
            ('v', '\\', _) => '>',
            ('^', '\\', _) => '<',
            ('<', '+', 1) =>  'v',
            ('>', '+', 1) =>  '^',
            ('v', '+', 1) =>  '>',
            ('^', '+', 1) =>  '<',
            ('<', '+', 0) =>  '^',
            ('>', '+', 0) =>  'v',
            ('v', '+', 0) =>  '<',
            ('^', '+', 0) =>  '>',
            _ => self.dir,
        };
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
    carts.sort();

    let mut i = 0;
    let mut collision = None;
    'outer: loop {
        if i >= carts.len() { break; }
        carts[i].step();
        let track = grid[carts[i].y][carts[i].x];
        carts[i].turn(track);

        let mut j = 0;
        loop {
            if j >= carts.len() { break; }
            if i != j && carts[i].collides(&carts[j]) {
                if collision == None {
                    collision = Some((carts[i].x, carts[i].y));
                }
                carts.remove(i);
                if j > i { j -= 1; }
                carts.remove(j);
                continue 'outer;
            }

            j += 1;
        }

        i += 1;
    }

    collision
}

pub fn run() {
    let file =  File::open("./input/day13.txt").expect("File not found");
    let input = BufReader::new(file);
    let mut carts = vec!();
    let mut grid = vec!();

    for (j, line) in input.lines().enumerate() {
        let mut tracks = vec!();
        for (i, c) in line.unwrap().chars().enumerate() {
            let cart = Cart{x: i, y: j, dir: c, i: 0 };
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
    part2(&grid, &mut carts);
}
