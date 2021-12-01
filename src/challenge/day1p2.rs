use std::collections::VecDeque;

pub fn run() {
    let (input, mut _aoc) = super::get(1);

    let mut increase_count = 0u32;
    let mut last_four = VecDeque::new();
    for line in input.lines() {
        let depth: u32 = line.parse().unwrap();

        last_four.push_back(depth);
        if last_four.len() > 4 {
            last_four.remove(0);
        }

        // println!("last four ({:?})", last_four);

        if last_four.len() == 4 {
            let w1 = last_four.range(0..3).fold(0, |a, v| a + v);
            let w2 = last_four.range(1..4).fold(0, |a, v| a + v);

            // println!("Window A: {} vs B: {}", w1, w2);
            if w2 > w1 {
                increase_count += 1;
            }
        }
    }

    println!("increase_count: {}", increase_count);
    // _aoc.submit(&increase_count.to_string()).unwrap();
}
