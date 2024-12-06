pub fn part_one(input: &str) -> u32 {
    let split = input.trim().split_once("\n\n").expect("invalid input");
    let rules = split.0;
    let updates = split.1;

    let mut map: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
    rules.lines().into_iter().for_each(|line| {
        let rule = line.split_once('|').expect("invalid input");
        let pg1: u32 = rule.0.parse().expect("not u32");
        let pg2: u32 = rule.1.parse().expect("not u32");
        if map.contains_key(&pg1) {
            let pages = map.get_mut(&pg1).expect("see above check");
            pages.push(pg2);
        } else {
            map.insert(pg1, vec![pg2]);
        }
    });

    updates.lines().into_iter().fold(0, |acc, line| {
        let update: Vec<u32> = line
            .split(',')
            .map(|x| x.parse().expect("not u32"))
            .collect();
        let pos = update
            .iter()
            .enumerate()
            .position(|(i, x)| match map.get(&x) {
                Some(val) => {
                    let previous = &update[0..i];
                    for item in val {
                        if previous.contains(item) {
                            return true;
                        }
                    }
                    return false;
                }
                None => return false,
            });
        if pos.is_none() {
            return acc + update[update.len() / 2];
        }
        return acc;
    })
}

pub fn part_two(input: &str) -> u32 {
    let split = input.trim().split_once("\n\n").expect("invalid input");
    let rules = split.0;
    let updates = split.1;

    let mut map: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
    rules.lines().into_iter().for_each(|line| {
        let rule = line.split_once('|').expect("invalid input");
        let pg1: u32 = rule.0.parse().expect("not u32");
        let pg2: u32 = rule.1.parse().expect("not u32");
        if map.contains_key(&pg1) {
            let pages = map.get_mut(&pg1).expect("see above check");
            pages.push(pg2);
        } else {
            map.insert(pg1, vec![pg2]);
        }
    });

    let is_valid_update = |update: &Vec<u32>| {
        let mut offending_num: u32 = 0;
        let pos = update
            .iter()
            .enumerate()
            .position(|(i, x)| match map.get(&x) {
                Some(val) => {
                    let previous = &update[0..i];
                    for item in val {
                        if previous.contains(item) {
                            offending_num = *item;
                            return true;
                        }
                    }
                    return false;
                }
                None => return false,
            });
        match pos {
            Some(p) => (false, p, offending_num),
            None => (true, 0, 0),
        }
    };

    updates.lines().into_iter().fold(0, |acc, line| {
        let mut update: Vec<u32> = line
            .split(',')
            .map(|x| x.parse().expect("not u32"))
            .collect();
        let (mut is_valid, mut pos, mut offending_num) = is_valid_update(&update);
        if is_valid {
            return acc;
        }
        loop {
            let idx = update
                .iter()
                .position(|x| *x == offending_num)
                .expect("should be there");
            let tmp = update[idx];
            update[idx] = update[pos];
            update[pos] = tmp;
            (is_valid, pos, offending_num) = is_valid_update(&update);
            if is_valid {
                return acc + update[update.len() / 2];
            }
        }
    })
}
