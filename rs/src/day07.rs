pub fn part_one(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| {
        let (test_val_str, num_strs) = line.split_once(": ").expect("invalid input");
        let nums: Vec<u64> = num_strs
            .split(' ')
            .map(|x| x.parse().expect("not u64"))
            .collect();
        let target: u64 = test_val_str.parse().expect("not u64");
        if can_make_target(&nums, target) {
            return acc + target;
        }
        acc
    })
}

pub fn part_two(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| {
        let (test_val_str, num_strs) = line.split_once(": ").expect("invalid input");
        let nums: Vec<u64> = num_strs
            .split(' ')
            .map(|x| x.parse().expect("not u64"))
            .collect();
        let target: u64 = test_val_str.parse().expect("not u64");
        if can_make_target_2(&nums, target) {
            return acc + target;
        }
        acc
    })
}

fn can_make_target(numbers: &[u64], target: u64) -> bool {
    fn evaluate_combinations(numbers: &[u64], current: u64, target: u64) -> bool {
        if numbers.is_empty() {
            return current == target;
        }

        let next_number = numbers[0];
        let remaining_numbers = &numbers[1..];

        if evaluate_combinations(remaining_numbers, current + next_number, target) {
            return true;
        }

        if evaluate_combinations(remaining_numbers, current * next_number, target) {
            return true;
        }

        false
    }

    if numbers.is_empty() {
        return false;
    }

    let initial = numbers[0];
    let remaining_numbers = &numbers[1..];
    evaluate_combinations(remaining_numbers, initial, target)
}

fn can_make_target_2(numbers: &[u64], target: u64) -> bool {
    fn evaluate_combinations(numbers: &[u64], current: u64, target: u64) -> bool {
        if numbers.is_empty() {
            return current == target;
        }

        let next_number = numbers[0];
        let remaining_numbers = &numbers[1..];

        if evaluate_combinations(remaining_numbers, current + next_number, target) {
            return true;
        }

        if evaluate_combinations(remaining_numbers, current * next_number, target) {
            return true;
        }

        if evaluate_combinations(
            remaining_numbers,
            (current.to_string() + &next_number.to_string())
                .parse()
                .expect("not u64"),
            target,
        ) {
            return true;
        }

        false
    }

    if numbers.is_empty() {
        return false;
    }

    let initial = numbers[0];
    let remaining_numbers = &numbers[1..];
    evaluate_combinations(remaining_numbers, initial, target)
}
