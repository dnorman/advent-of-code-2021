use aocf::Aoc;

automod::dir!(pub "src/challenge");

fn get(day: u32) -> (String, Aoc) {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(day)).init().unwrap();
    let input = aoc.get_input(false).expect("input").trim().to_string();
    return (input, aoc);
}
