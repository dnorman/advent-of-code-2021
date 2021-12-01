pub fn run() {
    let (input, mut _aoc) = super::get(1);
    let lines: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

    let increase_count = lines.windows(4).fold(0u32, |ac, window| {
        let a = window[0..3].iter().fold(0, |a, v| a + v);
        let b = window[1..4].iter().fold(0, |a, v| a + v);

        if b > a {
            ac + 1
        } else {
            ac
        }
    });

    println!("increase_count: {}", increase_count);
    // _aoc.submit(&increase_count.to_string()).unwrap();
}
