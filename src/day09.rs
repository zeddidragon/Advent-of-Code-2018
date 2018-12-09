fn game(player_count: usize, final_ball: usize) -> usize {
    let mut turn = player_count - 1;
    let mut current_index = 0;
    let mut ball = 0;
    let mut scores = vec![0; player_count];
    let mut balls = Vec::with_capacity(final_ball);
    balls.push(0);

    while ball < final_ball {
        let size = balls.len();
        turn = (turn + 1) % player_count;
        ball += 1;

        if ball % 23 > 0 {
            current_index = (current_index + 2) % size;
            if current_index == 0 {
                current_index = size;
            }
            balls.insert(current_index, ball);

            continue;
        }

        current_index = (current_index + size - 7) % size;
        let removed = balls.remove(current_index);
        scores[turn] += ball + removed;
    }

    let mut max = 0;
    for score in scores {
        if score > max {
            max = score;
        }
    }
    return max;
}

pub fn run() {
    assert_eq!(game(9, 25), 32);
    assert_eq!(game(10, 1618), 8317);
    assert_eq!(game(13, 7999), 146373);
    assert_eq!(game(17, 1104), 2764);
    assert_eq!(game(21, 6111), 54718);
    assert_eq!(game(30, 5807), 37305);
    
    // Too lazy to read input file
    let players = 464;
    let marble = 71730;

    println!("- Part 1 -");
    println!("Highest score is: {}", game(players, marble));

    println!("- Part 2 -");
    // println!("Highest score is: {}", game(players, marble * 100));
}
