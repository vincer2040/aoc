pub fn part_one(input: &str) -> u32 {
    input.trim().split('\n').into_iter().fold(0, |acc, cur| {
        let level: Vec<u32> = cur
            .split(' ')
            .map(|x| x.parse().expect("not a u32"))
            .collect();
        let mut acc_test = level.clone();
        let mut dec_test = level.clone();
        acc_test.sort();
        dec_test.sort_by(|a, b| b.cmp(a));
        if level == acc_test {
            let x = level.windows(2).position(|x| {
                let cur = x[0];
                let next = x[1];
                next - cur > 3 || next - cur < 1
            });
            match x {
                Some(_) => acc,
                None => acc + 1,
            }
        } else if level == dec_test {
            let x = level.windows(2).position(|x| {
                let cur = x[0];
                let next = x[1];
                cur - next > 3 || cur - next < 1
            });
            match x {
                Some(_) => acc,
                None => acc + 1,
            }
        } else {
            return acc;
        }
    })
}

pub fn part_two(input: &str) -> u32 {
    input.trim().split('\n').into_iter().fold(0, |acc, cur| {
        let mut level: Vec<u32> = cur
            .split(' ')
            .map(|x| x.parse().expect("not a u32"))
            .collect();
        let is_safe = |lvl: &Vec<u32>| {
            let mut acc_test = lvl.clone();
            let mut dec_test = lvl.clone();
            acc_test.sort();
            dec_test.sort_by(|a, b| b.cmp(a));
            if *lvl == acc_test {
                return lvl
                    .windows(2)
                    .position(|x| {
                        let cur = x[0];
                        let next = x[1];
                        next - cur > 3 || next - cur < 1
                    })
                    .is_none();
            } else if *lvl == dec_test {
                return lvl
                    .windows(2)
                    .position(|x| {
                        let cur = x[0];
                        let next = x[1];
                        cur - next > 3 || cur - next < 1
                    })
                    .is_none();
            } else {
                return false;
            }
        };
        if is_safe(&level) {
            return acc + 1;
        }
        let length = level.len();
        let mut old = 0;
        for i in 0..length {
            if i != 0 {
                level.insert(i - 1, old);
            }
            old = level[i];
            level.remove(i);
            if is_safe(&level) {
                return acc + 1;
            }
        }
        return acc;
    })
}
