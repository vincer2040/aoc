use std::io::Read;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

fn main() {
    let input = get_input();
    let result = day15::part_one(&input);
    println!("result: {}", result);
}

fn get_input() -> String {
    let mut input = Vec::new();
    std::io::stdin()
        .read_to_end(&mut input)
        .expect("failed to read");
    return String::from_utf8(input).expect("failed to parse utf8");
}
