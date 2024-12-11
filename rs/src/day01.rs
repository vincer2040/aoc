pub fn part_one(input: &str) -> u32 {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();
    input
        .split('\n')
        .into_iter()
        .filter(|line| {
            *line != ""
        })
        .for_each(|line| {
            let split: Vec<&str> = line.split("   ").collect();
            assert_eq!(split.len(), 2);
            let l: u32 = split[0].parse().expect("lhs not a u32");
            lhs.push(l);
            let r: u32 = split[1].parse().expect("rhs not a u32");
            rhs.push(r);
        });

    assert_eq!(lhs.len(), rhs.len());

    lhs.sort();
    rhs.sort();

    let mut result = 0;

    for i in 0..lhs.len() {
        let res = lhs[i].abs_diff(rhs[i]);
        result += res;
    }

    return result;
}

pub fn part_two(input: &str) -> u32 {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();
    input
        .split('\n')
        .into_iter()
        .filter(|line| {
            *line != ""
        })
        .for_each(|line| {
            let split: Vec<&str> = line.split("   ").collect();
            assert_eq!(split.len(), 2);
            let l: u32 = split[0].parse().expect("lhs not a u32");
            lhs.push(l);
            let r: u32 = split[1].parse().expect("rhs not a u32");
            rhs.push(r);
        });

    let mut map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();

    let mut result = 0;
    for item in lhs {
        match map.get(&item) {
            Some(value) => {
                result += item * value;
            }
            None => {
                let mut to_insert = 0;
                for item2 in &rhs {
                    if item == *item2 {
                        to_insert += 1;
                    }
                }
                map.insert(item, to_insert);
                result += item * to_insert;
            }
        }
    }
    return result;
}
