use regex::Regex;

pub fn part_one(input: &str) -> u32 {
    let mut result = 0;
    let re = Regex::new(r"mul\(\d+,\d+\)").expect("regex");
    for mat in re.find_iter(input) {
        let numre = Regex::new(r"\d+,\d+").expect("regex num");
        for m in numre.find_iter(mat.as_str()) {
            let nums = m.as_str().split_once(',').expect("two");
            let num1: u32 = nums.0.parse().expect("not u32");
            let num2: u32 = nums.1.parse().expect("not u32");
            result += num1 * num2;
        }
    }
    return result;
}

pub fn part_two(input: &str) -> u32 {
    let my_input = input
        .trim()
        .replace("\n", "")
        .replace("do()", "\ndo()")
        .replace("don't()", "\ndon't()");
    let filtered: Vec<&str> = my_input
        .lines()
        .filter(|line| !line.starts_with("don't()"))
        .collect();
    filtered.iter().fold(0, |acc, line| {
        let re = Regex::new(r"mul\(\d+,\d+\)").expect("regex");
        let mut cur_line_amt = 0;
        for mat in re.find_iter(line) {
            let numre = Regex::new(r"\d+,\d+").expect("regex num");
            for m in numre.find_iter(mat.as_str()) {
                let nums = m.as_str().split_once(',').expect("two");
                let num1: u32 = nums.0.parse().expect("not u32");
                let num2: u32 = nums.1.parse().expect("not u32");
                cur_line_amt += num1 * num2
            }
        }
        return acc + cur_line_amt;
    })
}
