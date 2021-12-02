pub fn run() {
    let (input, mut _aoc) = super::get(2);

    let mut aim = 0i32;
    let mut horiz = 0i32;
    let mut depth = 0i32;
    for line in input.lines() {
        let mut f = line.split(" ");
        let dir = f.next().unwrap();
        let units: i32 = f.next().unwrap().parse().unwrap();

        println!("{}: {}", dir, units);
        match dir {
            "up" => aim -= units,
            "down" => aim += units,
            "forward" => {
                horiz += units;
                depth += aim * units;
            }
            _ => unimplemented!(),
        }
    }

    _aoc.submit(&(horiz * depth).to_string()).unwrap();
}
