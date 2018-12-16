mod particle;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

struct Advent {
    day: i8
}

fn next(calendar: &mut Advent) {
    calendar.day += 1;
    println!("\n# Day {}", calendar.day);
}

fn main() {
    let mut day = Advent { day: 0 };
    println!("## Advent of Code 2018 ##");

    next(&mut day);
    day01::run();

    next(&mut day);
    day02::run();

    next(&mut day);
    day03::run();

    next(&mut day);
    day04::run();

    next(&mut day);
    day05::run();

    next(&mut day);
    day06::run();

    next(&mut day);
    day07::run();

    next(&mut day);
    day08::run();

    next(&mut day);
    day09::run();

    next(&mut day);
    day10::run();

    next(&mut day);
    day11::run();

    next(&mut day);
    day12::run();

    next(&mut day);
    println!("-- Not until tomorrow! --");
}
