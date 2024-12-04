use std::io::Read;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn main() {
    let input = get_input();
    let result = day4::part_two(&input);
    println!("result: {}", result);
}

fn get_input() -> String {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).expect("failed to read");
    return String::from_utf8(input).expect("failed to parse utf8");
}
