const WIDTH: u8 = 12;
pub fn run() {
    let (input, mut _aoc) = super::get(3);

    let lines: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    let oxygen_gen_rating = get_line_with_most_matching_msb(lines.clone(), false).unwrap();
    let co2_scrubber_rating = get_line_with_most_matching_msb(lines.clone(), true).unwrap();

    let life_support_rating = oxygen_gen_rating * co2_scrubber_rating;

    assert_eq!(life_support_rating, 5736383);
}

fn get_line_with_most_matching_msb(mut lines: Vec<usize>, invert: bool) -> Result<usize, ()> {
    //iterate over bit positions from msb to lsb
    for bit in (0..WIDTH).rev() {
        // get the average value over all lines for this bit offset. Rounding up in the case of a tie
        let mut match_bit = avg_bit_round_up(&lines, bit);
        if invert {
            match_bit = !match_bit;
        }

        // throw out any lines that don't match the average bit
        lines.retain(|line| match_bit == query_bit(*line, bit));

        if lines.len() == 1 {
            return Ok(lines[0]);
        }
    }
    Err(())
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
