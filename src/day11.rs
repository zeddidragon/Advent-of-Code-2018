pub fn best_square(grid: &Vec<i32>, size: usize) -> (i32, usize, usize) {
    let width = 300;
    let height = 300;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_power = std::i32::MIN;
    let mut cache : Vec<i32> = vec![0; size];
    'outer: for x in 0..(width + 1 - size) {
        cache[0] = 0;
        for (y, value) in cache.iter_mut().skip(1).enumerate() {
            *value = grid
                .iter()
                .skip(x + y * width)
                .take(size)
                .sum();
        }
        let mut power = cache.iter().sum();
        let mut cache_index = 0;

        for y in 0..(height + 1 - size) {
            let value = cache[cache_index];
            let new_value : i32 = grid
                .iter()
                .skip(x + (y + size - 1) * width)
                .take(size)
                .sum();
            power += new_value - value;
            cache[cache_index] = new_value;
            cache_index = (cache_index + 1) % size;

            if power > max_power {
                max_power = power;
                max_x = x;
                max_y = y;
            }
        }
    }

    (max_power, max_x + 1, max_y + 1)
}

pub fn part1(grid: &Vec<i32>) {
    let (power, x, y) = best_square(grid, 3);
    println!("Max power is {} ({}, {})", power, x, y);
}

pub fn part2(grid: &Vec<i32>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;
    let mut max_power = 0;
    for size in 1..20 {  // Given the examples (and me checking), it's not larger than 20
        let (power, x, y) = best_square(grid, size);
        if power > max_power {
            max_power = power;
            max_x = x;
            max_y = y;
            max_size = size;
        }
    }
    println!("Max power is {} ({}, {}, {})", max_power, max_x, max_y, max_size);
}

pub fn run() {
    let serial = 7803; // Puzzle input
    let width = 300;
    let mut grid = vec![0; 300 * 300];

    for (i, value) in grid.iter_mut().enumerate() {
        let x = (i % width + 1) as i32; // 1-based coords
        let y = (i / width + 1) as i32;
        let id = x + 10;

        *value = (((id * y + serial) * id) % 1000) / 100 - 5;
    }

    println!("- Part 1 -");
    part1(&grid);
    println!("- Part 2 -");
    part2(&grid);
}
