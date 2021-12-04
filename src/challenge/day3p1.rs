pub fn run() {
    let (input, mut _aoc) = super::get(3);

    let mut bit_counts = [0u32; 12];
    let mut total_lines = 0u32;

    for line in input.lines() {
        let num = isize::from_str_radix(line, 2).unwrap();
        total_lines += 1;
        // This is a text representation of binary, thus the least significant bit is to the right
        for i in 0..12 {
            let one = (num & 1 << i) != 0;
            bit_counts[i] += one as u32;
        }
        // println!("{}", line);
    }

    let mut gamma_rate = 0i32;
    for (i, ones) in bit_counts.iter().enumerate() {
        let zeros = total_lines - ones;

        if *ones > zeros {
            gamma_rate |= 0b1 << i;
        }
    }

    let epsilon_rate = gamma_rate ^ 0b111111111111;

    // println!("{:b}, {:b}", gamma_rate, epsilon_rate);

    let power_consumption = gamma_rate * epsilon_rate;

    assert_eq!(power_consumption, 3277364);
    // _aoc.submit(&("whatever").to_string()).unwrap();
}
