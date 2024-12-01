use std::io::Read;

mod day1;

fn main() {
    let input = get_input();
    let result = day1::part_two(&input);
    println!("result: {}", result);
}

fn get_input() -> String {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).expect("failed to read");
    return String::from_utf8(input).expect("failed to parse utf8");
}
