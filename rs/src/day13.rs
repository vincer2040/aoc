#[derive(Debug)]
struct Button {
    cost: i64,
    dx: i64,
    dy: i64,
}

#[derive(Debug)]
struct Target {
    x: i64,
    y: i64,
}

const BUTTON_A_COST: i64 = 3;
const BUTTON_B_COST: i64 = 1;

pub fn part_one(input: &str) -> i64 {
    let games = parse_games(input, true);
    games.iter().fold(0, |acc, (btn_a, btn_b, target)| {
        let max_na = target.x / btn_a.dx + 1;
        let max_nb = target.x / btn_b.dx + 1;

        let mut min_cost = i64::MAX;

        for na in 0..max_na {
            for nb in 0..max_nb {
                let x_reached = na * btn_a.dx + nb * btn_b.dx;
                let y_reached = na * btn_a.dy + nb * btn_b.dy;

                if x_reached == target.x && y_reached == target.y {
                    let cost = na * btn_a.cost + nb * btn_b.cost;
                    if cost < min_cost {
                        min_cost = cost;
                    }
                }
            }
        }

        if min_cost < i64::MAX {
            return acc + min_cost;
        } else {
            return acc;
        }
    })
}

pub fn part_two(input: &str) -> i64 {
    let games = parse_games(input, false);
    games.iter().fold(0, |acc, (btn_a, btn_b, target)| {
        let denom = (btn_a.dy * btn_b.dx) - (btn_a.dx * btn_b.dy);
        assert_ne!(denom, 0, "we only handle linearly independent buttons");
        let num = (target.y * btn_b.dx) - (target.x * btn_b.dy);
        let a_rem = num % denom;

        if a_rem == 0 {
            let a_count = num / denom;
            let b_total = (target.x - btn_a.dx * a_count) / btn_b.dx;
            return acc + (a_count * 3) + b_total;
        }
        return acc;
    })
}

fn parse_games(input: &str, part_one: bool) -> Vec<(Button, Button, Target)> {
    let games: Vec<(Button, Button, Target)> = input
        .trim()
        .split("\n\n")
        .map(|game| {
            let items: Vec<&str> = game.lines().collect();
            let btn_a = items[0];
            let btn_b = items[1];
            let target = items[2];

            let (_, btn_a_actions) = btn_a.split_once(": ").unwrap();
            let (btn_a_x_dec, btn_a_y_dec) = btn_a_actions.split_once(", ").unwrap();
            let (_, btn_a_x) = btn_a_x_dec.split_once('+').unwrap();
            let (_, btn_a_y) = btn_a_y_dec.split_once('+').unwrap();

            let (_, btn_b_actions) = btn_b.split_once(": ").unwrap();
            let (btn_b_x_dec, btn_b_y_dec) = btn_b_actions.split_once(", ").unwrap();
            let (_, btn_b_x) = btn_b_x_dec.split_once('+').unwrap();
            let (_, btn_b_y) = btn_b_y_dec.split_once('+').unwrap();

            let (_, target_location) = target.split_once(": ").unwrap();
            let (target_x_dec, target_y_dec) = target_location.split_once(", ").unwrap();
            let (_, target_x) = target_x_dec.split_once('=').unwrap();
            let (_, target_y) = target_y_dec.split_once('=').unwrap();

            let a = Button {
                cost: BUTTON_A_COST,
                dx: btn_a_x.parse().unwrap(),
                dy: btn_a_y.parse().unwrap(),
            };
            let b = Button {
                cost: BUTTON_B_COST,
                dx: btn_b_x.parse().unwrap(),
                dy: btn_b_y.parse().unwrap(),
            };
            let target = if part_one {
                Target {
                    x: target_x.parse().unwrap(),
                    y: target_y.parse().unwrap(),
                }
            } else {
                Target {
                    x: target_x.parse::<i64>().unwrap() + 10000000000000,
                    y: target_y.parse::<i64>().unwrap() + 10000000000000,
                }
            };
            return (a, b, target);
        })
        .collect();
    return games;
}
