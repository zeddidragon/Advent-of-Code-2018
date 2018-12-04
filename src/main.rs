mod day01;
mod day02;
mod day03;
mod day04;

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
    println!("-- Not until tomorrow! --");
}
