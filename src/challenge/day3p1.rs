pub fn run() {
    let (input, mut _aoc) = super::get(3);

    for line in input.lines() {
        println!("{}", line);
    }

    _aoc.submit(&("whatever").to_string()).unwrap();
}
