use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let (towels, combos) = parse_input(input);
    combos.iter().fold(0, |acc, cur| {
        return acc + ((num_possible(&towels, cur.as_bytes(), &mut HashMap::new()) > 0) as u32) ;
    })
}

pub fn part_two(input: &str) -> u64 {
    let (towels, combos) = parse_input(input);
    combos.iter().fold(0, |acc, cur| {
        return acc + num_possible(&towels, cur.as_bytes(), &mut HashMap::new());
    })
}

fn num_possible<'d>(
    patterns: &[&[u8]],
    design: &'d [u8],
    memo: &mut HashMap<&'d [u8], u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(&num_possibilities) = memo.get(design) {
        return num_possibilities;
    }

    let num_possibilities = patterns
        .iter()
        .filter(|p| design.starts_with(p))
        .map(|p| num_possible(patterns, &design[p.len()..], memo))
        .sum();

    memo.insert(design, num_possibilities);
    num_possibilities
}

fn parse_input(input: &str) -> (Vec<&[u8]>, Vec<&str>) {
    let (towels_str, combos_str) = input.trim().split_once("\n\n").expect("invalid input");
    let towels: Vec<&[u8]> = towels_str.split(", ").map(|t| t.as_bytes()).collect();
    let combos: Vec<&str> = combos_str.lines().collect();
    (towels, combos)
}
