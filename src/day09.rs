struct Ball {
    next: usize,
    prev: usize,
}

fn game(player_count: usize, final_ball: usize) -> usize {
    let mut turn = player_count - 1;
    let mut ball = 0;
    let mut current = 0;
    let mut scores = vec![0; player_count];
    let mut balls = Vec::new();
    for _ in 0..(final_ball + 1) {
        balls.push(Ball { next: 0, prev: 0 });
    }

    while ball < final_ball {
        turn = (turn + 1) % player_count;
        ball += 1;
        let prev;
        let next;

        if ball % 23 > 0 {
            {
                prev = balls.get(current).unwrap().next;
            }
            {
                next = balls.get(prev).unwrap().next;
            }
            {
                let between = balls.get_mut(ball).unwrap();
                between.prev = prev;
                between.next = next;
            }
            {
                let before = balls.get_mut(prev).unwrap();
                before.next = ball;
            }
            {
                let after = balls.get_mut(next).unwrap();
                after.prev = ball;
            }
            current = ball;
            continue;
        }

        for _ in 0..7 {
            current = balls.get(current).unwrap().prev;
        }
        scores[turn] += ball + current;

        {
            let removed = balls.get(current).unwrap();
            prev = removed.prev;
            next = removed.next;
            current = removed.next;
        }
        {
            let before = balls.get_mut(prev).unwrap();
            before.next = next;
        }
        {
            let after = balls.get_mut(next).unwrap();
            after.prev = prev;
        }
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
    let marbles = 71730;

    println!("- Part 1 -");
    println!("Highest score is: {}", game(players, marbles));

    println!("- Part 2 -");
    println!("Highest score is: {}", game(players, marbles * 100));
}
