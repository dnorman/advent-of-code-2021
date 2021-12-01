pub fn run() {
    let (input, mut _aoc) = super::get(1);
    let lines: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

    let increase_count =
        lines.windows(2).fold(
            0u32,
            |ac, window| {
                if window[1] > window[0] {
                    ac + 1
                } else {
                    ac
                }
            },
        );
    println!("increase_count: {}", increase_count);
    // _aoc.submit(&increase_count.to_string()).unwrap();
}
