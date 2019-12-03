struct Cursor {
    idx: usize
}

impl Cursor {
    fn val(&self, xs: &Vec<i8>) -> i8 {
        xs[self.idx]
    }

    fn spin(&mut self, xs: &Vec<i8>) {
        self.idx = (self.idx + 1 + self.val(xs) as usize) % xs.len();
    }
}

fn print_slice(slice: &[i8]) {
    for i in slice {
        print!("{}", i)
    }
    println!("");
}

fn seek(
    mut scores: &mut Vec<i8>,
    mut i: &mut Cursor,
    mut j: &mut Cursor,
    pattern: &[i8]
) -> usize {
    let mut offset = 0;
    'outer: loop {
        while offset + pattern.len() > scores.len() {
            step(&mut scores, &mut i, &mut j)
        }
        for (i, &v) in pattern.iter().enumerate() {
            if v != scores[i + offset] {
                offset += 1;
                continue 'outer;
            }
        }
        return offset;
    }
}

fn step(scores: &mut Vec<i8>, i: &mut Cursor, j: &mut Cursor) {
    let mix = i.val(&scores) + j.val(&scores);
    if mix >= 10 { scores.push(1); }
    scores.push(mix % 10);

    i.spin(&scores);
    j.spin(&scores);
}

pub fn run() {
    let input = 323081; // Puzzle input
    let mut scores = vec![3, 7];
    let mut i = Cursor{idx: 0};
    let mut j = Cursor{idx: 1};

    println!("- Part 1 -");
    while scores.len() < input + 10 {
        step(&mut scores, &mut i, &mut j);
    }
    print_slice(&scores[input..input + 10]);
    println!("- Part 2 -");
    let pattern = [3, 2, 3, 0, 8, 1];
    let pos = seek(&mut scores, &mut i, &mut j, &pattern);
    println!("Match at {}", pos);
}
