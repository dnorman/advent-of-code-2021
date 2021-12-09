const WIDTH: u8 = 12;
pub fn run() {
    let (input, mut _aoc) = super::get(3);

    //     let input = "00100
    // 11110
    // 10110
    // 10111
    // 10101
    // 01111
    // 00111
    // 11100
    // 10000
    // 11001
    // 00010
    // 01010"
    //         .to_string();

    let lines: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    // The bit criteria depends on which type of rating value you want to find:

    // To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position,
    // and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with
    // a 1 in the position being considered.

    let mut oxygen_generator_rating = 0usize;
    {
        let mut og_lines = lines.clone();
        'og_scan: for bit in (0..WIDTH).rev() {
            // Add one bit at a time to the bit_criteria

            let (avg_bits_up, _) = avg_bits(&og_lines);
            let match_bit = query_bit(avg_bits_up, bit);
            println!("OG scan {}: {:b},{:?}", bit, avg_bits_up, match_bit);

            og_lines.retain(|line| match_bit == query_bit(*line, bit));

            for line in og_lines.iter() {
                println!("\t{:05b} ", line);
            }
            if og_lines.len() == 1 {
                oxygen_generator_rating = og_lines[0];
                break 'og_scan;
            }
        }
    }

    println!("oxygen_generator_rating: {}", oxygen_generator_rating);

    // To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position,
    // and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with
    // a 0 in the position being considered.

    // Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
    // If you only have one number left, stop; this is the rating value for which you are searching.
    // Otherwise, repeat the process, considering the next bit to the right.

    let mut co2_scrubber_rating = 0usize;
    {
        let mut co2_lines = lines.clone();
        'co2_scan: for bit in (0..WIDTH).rev() {
            // Add one bit at a time to the bit_criteria

            let (avg_bits_up, _) = avg_bits(&co2_lines);
            let match_bit = !query_bit(avg_bits_up, bit);
            println!("CO2 scan {}: {:b},{:?}", bit, avg_bits_up, match_bit);

            co2_lines.retain(|line| match_bit == query_bit(*line, bit));

            for line in co2_lines.iter() {
                println!("\t{:05b} ", line);
            }
            if co2_lines.len() == 1 {
                co2_scrubber_rating = co2_lines[0];
                break 'co2_scan;
            }
        }
    }

    println!("co2_scrubber_rating: {}", co2_scrubber_rating);

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    assert_eq!(life_support_rating, 1);
    // _aoc.submit(&("whatever").to_string()).unwrap();
}

fn query_bit(num: usize, bit: u8) -> bool {
    ((num >> bit) & 1) != 0
}

enum Match {
    One(usize),
    Many,
    None,
}

// Is binary averaging like, a remotely normal thing to do?
// Is this some kind of super common bitwise operation in disguise?
// or are they just fucking with us?

/// Return a usize by which each bit has been selected by averaging that bit over the input list
/// Ties among each bit are assigned a bit value of 0.
fn avg_bits(lines: &[usize]) -> (usize, usize) {
    let mut bit_counts = [0usize; 12];
    for line in lines {
        for i in 0..12 {
            let one = (line >> i & 1) != 0;
            bit_counts[i] += one as usize; // cast boolean to 0 or 1
        }
    }

    let total_lines = lines.len();
    let mut avg_bits_round_up = 0usize;
    let mut avg_bits_round_down = 0usize;
    for (i, ones) in bit_counts.iter().enumerate() {
        let zeros = total_lines - ones;

        if *ones > zeros {
            avg_bits_round_up |= (0b1 << i);
            avg_bits_round_down |= (0b1 << i);
        } else if *ones == zeros {
            avg_bits_round_up |= (0b1 << i);
        }
        // (else) round down is the default
    }

    (avg_bits_round_up, avg_bits_round_down)
}
