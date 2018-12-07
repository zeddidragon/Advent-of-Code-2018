use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;
use std::usize::MAX;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}

struct DistanceResult {
    closest_index: usize,
    total_distance: usize,
    surrounded: bool,
    tied: bool,
}

fn distances(points: &Vec<Point>, n: &Point) -> DistanceResult {
    let mut closest = 0;
    let mut closest_distance = MAX;
    let mut total_distance = 0;
    let mut tied = MAX;
    let mut left = false;
    let mut right = false;
    let mut above = false;
    let mut below = false;

    for (i, point) in points.iter().enumerate() {
        if !left && point.x < n.x { left = true; }
        if !right && point.x > n.x { right = true; }
        if !above && point.y < n.y { above = true; }
        if !below && point.y > n.y { below = true; }

        let distance = (
            (point.x - n.x).abs() +
            (point.y - n.y).abs()
        ) as usize;

        total_distance += distance;

        if distance < closest_distance {
            closest = i;
            closest_distance = distance;
        } else if distance == closest_distance {
            tied = distance;
        }
    }

    return DistanceResult {
        closest_index: closest,
        total_distance: total_distance,
        tied: tied == closest_distance,
        surrounded: left && right && above && below,
    }
}

fn add_neighbours(neighbours: &mut Vec<Point>, n: &Point) {
    neighbours.push(Point { x: n.x - 1, y: n.y });
    neighbours.push(Point { x: n.x + 1, y: n.y });
    neighbours.push(Point { x: n.x, y: n.y - 1 });
    neighbours.push(Point { x: n.x, y: n.y + 1 });
}

fn part1(points: &Vec<Point>) {
    let mut visited : HashSet<Point> = HashSet::new();
    let mut neighbours : Vec<Point> = Vec::new();
    let mut largest = 0;
    let mut largest_index = 0;

    'points: for (i, point) in points.iter().enumerate() {
        let mut size = 0;
        visited.clear();
        neighbours.clear();
        neighbours.push(point.clone());

        'search: while !neighbours.is_empty() {
            let n = neighbours.pop().unwrap();
            
            if visited.contains(&n) {
                continue 'search;
            }
            visited.insert(n);

            let result = distances(points, &n);

            if result.closest_index != i || result.tied {
                continue 'search;
            }

            // Area is infinite
            if !result.surrounded {
                continue 'points;
            }

            size += 1;
            add_neighbours(&mut neighbours, &n);
        }

        if size > largest {
            largest = size;
            largest_index = i;
        }
    }

    println!("Largest area belongs to vector #{}, (size {})", largest_index, largest);
}

fn part2(points: &Vec<Point>) {
    let mut left = MAX;
    let mut right = 0;
    let mut top = MAX;
    let mut bottom = 0;
    for point in points {
        if (point.x as usize) < left {
            left = point.x as usize;
        }
        if (point.x as usize) > right {
            right = point.x as usize;
        }
        if (point.y as usize) < top {
            top = point.y as usize;
        }
        if (point.y as usize) > bottom {
            bottom = point.y as usize;
        }
    }

    let mut visited : HashSet<Point> = HashSet::new();
    let mut neighbours = vec!(Point {
        x: (left + right) as i16 / 2,
        y: (top + bottom) as i16 / 2,
    });
    let mut size = 0;

    
    'search: while !neighbours.is_empty() {
        let n = neighbours.pop().unwrap();
        
        if visited.contains(&n) {
            continue 'search;
        }
        visited.insert(n);
        let result = distances(points, &n);

        if result.total_distance < 10_000 {
            size += 1;
            add_neighbours(&mut neighbours, &n);
        }
    }

    println!("Area has a size of {}", size);
}

fn to_point(line: &str) -> Point {
    let mut coords = line
        .split(", ")
        .map(|p| p.parse().unwrap());
    return Point {
        x: coords.next().unwrap(),
        y: coords.next().unwrap(),
    }
}

pub fn run() {
    let file =  File::open("./input/day06.txt").expect("File not found");
    let input = BufReader::new(file);
    let points : Vec<Point> = input
        .lines()
        .map(|l| to_point(&l.unwrap()))
        .collect();

    println!("- Part 1 -");
    part1(&points);
    println!("- Part 2 -");
    part2(&points);
}
