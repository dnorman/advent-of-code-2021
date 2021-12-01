pub fn run() {
    let (input, mut _aoc) = super::get(1);

    let mut increase_count = 0u32;
    let mut last_depth = None;
    for line in input.lines() {
        let depth: u32 = line.parse().unwrap();

        match last_depth {
            None => {}
            Some(l) => {
                if depth > l {
                    increase_count += 1;
                }
            }
        }
        last_depth = Some(depth)
    }

    println!("increase_count: {}", increase_count);
    // _aoc.submit(&increase_count.to_string()).unwrap();
}
