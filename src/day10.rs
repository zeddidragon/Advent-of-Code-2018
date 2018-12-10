extern crate regex;
use self::regex::{Regex};
use std::io::{BufReader, BufRead};
use std::fs::File;
use particle::{Particle, Point};

fn bounds(particles: &Vec<Particle>) -> (Point, Point) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;

    for mut p in particles.iter() {
        let x = p.pos.x;
        let y = p.pos.y;

        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    (Point {x: min_x, y: min_y}, Point {x: max_x, y: max_y})
}

fn part1(particles: &mut Vec<Particle>) -> u32 {
    // We're not sure how tall a letter is.
    // We're looking for the moment when the points are closest together
    // in the y-axis.
    //
    // Iterate until height starts growing, then take one step back

    let width;
    let mut height = std::i32::MAX;
    let mut seconds = 0;
    loop {
        for mut p in particles.iter_mut() {
            p.pos += p.vel;
        }

        let (min, max) = bounds(&particles);

        if max.y - min.y < height {
            seconds += 1;
            height = max.y - min.y;
            continue;
        }

        for mut p in particles.iter_mut() {
            p.pos -= p.vel;
        }
        
        let (min, max) = bounds(&particles);

        for mut p in particles.iter_mut() {
            p.pos -= min;
        }

        width = max.x - min.x + 1;
        height = max.y - min.y + 1;
        break;
    }

    let mut grid = vec![false; (width * height) as usize];
    let mut buf = String::with_capacity((2 * width * height + height) as usize);

    for p in particles {
        let index = (p.pos.x + p.pos.y * width) as usize;
        *grid.get_mut(index).unwrap() = true;
    }

    for (i, node) in grid.iter().enumerate() {
        if i > 0 && i % (width as usize) == 0 {
            buf.push('\n');
        } else if i > 0 {
            buf.push(' ');
        }
        if *node {
            buf.push('*');
        } else {
            buf.push(' ');
        }
    }
    println!("{}", buf);

    seconds
}

pub fn run() {
    let num = r"\s*(-?\d+)";
    let pattern = format!("^position=<{0},{0}> velocity=<{0},{0}>$", num);
    let rex = Regex::new(&pattern).unwrap();
    let file =  File::open("./input/day10.txt").expect("File not found");
    let input = BufReader::new(file);
    let mut particles : Vec<Particle> = input
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let caps = rex.captures_iter(&line).next().unwrap();
            Particle {
                pos: Point {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                },
                vel: Point {
                    x: caps[3].parse().unwrap(),
                    y: caps[4].parse().unwrap(),
                },
            }
        })
        .collect();

    println!("- Part 1 -");
    let steps = part1(&mut particles);
    println!("- Part 2 -");
    println!("Simulation took {} steps", steps);
}
