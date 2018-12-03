use std::io::{BufReader, BufRead};
use std::fs::File;

const WIDTH : usize = 1000;
struct Claim {
    id: u16,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Claim {
    fn to_string(&self) -> String {
        return format!(
            "#{} @ {},{}: {}x{}",
            self.id,
            self.x,
            self.y,
            self.w,
            self.h,
        );
    }
}

fn part1(grid: &[u16; 1_000_000]) {
    let mut disputed = 0;
    for square in grid.iter() {
        if *square > 1 {
            disputed += 1;
        }
    }

    println!("Found {} disputed squares", disputed);
}

fn part2(grid: &[u16; 1_000_000], claims: &Vec<Claim>) {
    for claim in claims.iter() {
        let mut collision = false;
        'outer: for j in claim.y..(claim.y + claim.h) {
            for i in claim.x..(claim.x + claim.w) {
                let value = grid[i + j * WIDTH];
                if value != 1 {
                    collision = true;
                    break 'outer;
                }
            }
        }
        if !collision {
            println!("No collisions for claim {}", claim.to_string());
            return ();
        }
    }
}

fn to_claim(line: &str) -> Claim {
    let mut id = 0;
    let mut x = 0;
    let mut y = 0;
    let mut w = 0;
    let mut h = 0;

    for (i, part) in line.split_whitespace().enumerate() {
        match i {
            0 => id = part[1..].parse().unwrap(),
            2 => {
                let parts : Vec<usize> = part[..part.len() - 1]
                    .split(",")
                    .map(|p| p.parse().unwrap())
                    .collect();
                x = parts[0];
                y = parts[1];
            }
            3 => {
                let parts : Vec<usize> = part
                    .split("x")
                    .map(|p| p.parse().unwrap())
                    .collect();
                w = parts[0];
                h = parts[1];
            }
            _ => ()
        }
    }

    return Claim {
        id: id,
        x: x,
        y: y,
        w: w,
        h: h,
    };
}

pub fn run() {
    let file =  File::open("./input/day03.txt").expect("File not found");
    let input = BufReader::new(file);
    let claims : Vec<Claim> = input
        .lines()
        .map(|l| to_claim(&l.unwrap()))
        .collect();

    let mut grid : [u16; 1000_000] = [0; 1000_000];
    for claim in claims.iter() {
        for j in claim.y..(claim.y + claim.h) {
            for i in claim.x..(claim.x + claim.w) {
                grid[i + j * WIDTH] += 1;
            }
        }
    }

    println!("- Part 1 -");
    part1(&grid);
    println!("- Part 2 -");
    part2(&grid, &claims);
}
