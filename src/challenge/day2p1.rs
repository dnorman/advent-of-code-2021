pub fn run() {
    let (input, mut _aoc) = super::get(2);

    let mut horiz = 0i32;
    let mut depth = 0i32;
    for line in input.lines() {
        let mut f = line.split(" ");
        let dir = f.next().unwrap();
        let dist: i32 = f.next().unwrap().parse().unwrap();

        println!("{}: {}", dir, dist);
        match dir {
            "up" => depth -= dist,
            "down" => depth += dist,
            "forward" => horiz += dist,
            _ => unimplemented!(),
        }
    }

    _aoc.submit(&(horiz * depth).to_string()).unwrap();
}
