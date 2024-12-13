use cached::cached;

pub fn part_one(input: &str) -> u32 {
    let mut stones: Vec<u64> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    for _ in 0..25 {
        stones = stones.iter().fold(Vec::new(), |mut acc, stone| {
            match stone {
                0 => {
                    acc.push(1);
                }
                _ if stone.to_string().len() % 2 == 0 => {
                    let len = (*stone as f64).log10() as usize + 1;
                    let half = 10_u64.pow((len / 2) as u32);

                    let left = stone / half;
                    let right = stone % half;

                    if left > 0 {
                        acc.push(left);
                    } else {
                        acc.push(0);
                    }

                    if right > 0 {
                        acc.push(right);
                    } else {
                        acc.push(0);
                    }
                }
                _ => {
                    acc.push(stone * 2024);
                }
            }
            return acc;
        });
    }
    return stones.len() as u32;
}

pub fn part_two(input: &str) -> u64 {
    let stones: Vec<u64> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    stones
        .iter()
        .fold(0, |acc, stone| acc + count_stone_blinks(*stone, 75))
}

cached! {
    BLINKONE;
    fn blink_one_stone(stone: u64) -> (u64, Option<u64>) = {
        match stone {
            0 => (1, None),
            _ if stone.to_string().len() % 2 == 0 => {
                let len = (stone as f64).log10() as usize + 1;
                let half = 10_u64.pow((len / 2) as u32);

                let left = stone / half;
                let right = stone % half;

                (left, Some(right))
            }
            _ => (stone * 2024, None),
        }
    }
}

cached! {
    COUNT;
    fn count_stone_blinks(stone: u64, depth: u64) -> u64 = {
        let (left, right_opt) = blink_one_stone(stone);
        if depth == 1 {
            if right_opt.is_some() {
                2
            } else {
                1
            }
        } else {
            let mut output = count_stone_blinks(left, depth - 1);
            if let Some(right) = right_opt {
                output += count_stone_blinks(right, depth - 1);
            }
            output
        }
    }
}
