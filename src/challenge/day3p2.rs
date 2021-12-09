const WIDTH: u8 = 12;
pub fn run() {
    let (input, mut _aoc) = super::get(3);

    let lines: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    let mut oxygen_generator_rating = 0usize;
    {
        let mut og_lines = lines.clone();
        'og_scan: for bit in (0..WIDTH).rev() {
            let match_bit = avg_bit_round_up(&og_lines, bit);

            og_lines.retain(|line| match_bit == query_bit(*line, bit));

            if og_lines.len() == 1 {
                oxygen_generator_rating = og_lines[0];
                break 'og_scan;
            }
        }
    }

    let mut co2_scrubber_rating = 0usize;
    {
        let mut co2_lines = lines.clone();
        'co2_scan: for bit in (0..WIDTH).rev() {
            let match_bit = !avg_bit_round_up(&co2_lines, bit);

            co2_lines.retain(|line| match_bit == query_bit(*line, bit));

            if co2_lines.len() == 1 {
                co2_scrubber_rating = co2_lines[0];
                break 'co2_scan;
            }
        }
    }

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    assert_eq!(life_support_rating, 5736383);
}

fn query_bit(num: usize, bit: u8) -> bool {
    ((num >> bit) & 1) != 0
}

// get the average value of the specified bit in the list, rounding up in cases of a tie
fn avg_bit_round_up(lines: &[usize], bit: u8) -> bool {
    let mut ones = 0usize;
    for line in lines {
        if (line >> bit & 1) != 0 {
            ones += 1
        }
    }

    let zeros = lines.len() - ones;
    return ones >= zeros;
}

// Is binary averaging like, a remotely normal thing to do?
// Is this some kind of super common bitwise operation in disguise?
// or are they just fucking with us?
