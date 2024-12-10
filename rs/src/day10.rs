use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

struct Dir {
    dx: isize,
    dy: isize,
}

pub fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let max_y = grid.len() as isize;
    let max_x = grid[0].len() as isize;
    let start_positions = get_start_positions(&grid);
    start_positions.iter().fold(0, |acc, pos| {
        let score = walk(
            &grid,
            *pos,
            max_x,
            max_y,
            -1,
            &mut Some(&mut HashSet::new()),
        );
        return acc + score;
    })
}

pub fn part_two(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let max_y = grid.len() as isize;
    let max_x = grid[0].len() as isize;
    let start_positions = get_start_positions(&grid);
    start_positions.iter().fold(0, |acc, pos| {
        let score = walk(&grid, *pos, max_x, max_y, -1, &mut None);
        return acc + score;
    })
}

const DIRS: [Dir; 4] = [
    Dir { dx: 0, dy: -1 },
    Dir { dx: 1, dy: 0 },
    Dir { dx: 0, dy: 1 },
    Dir { dx: -1, dy: 0 },
];

fn walk(
    grid: &Vec<Vec<char>>,
    position: Point,
    max_x: isize,
    max_y: isize,
    last_num: i8,
    seen: &mut Option<&mut HashSet<Point>>,
) -> u32 {
    if position.x < 0 || position.x >= max_x || position.y < 0 || position.y >= max_y {
        return 0;
    }
    let val = grid[position.y as usize][position.x as usize] as i8 - '0' as i8;
    if val != last_num + 1 {
        return 0;
    }
    if val == 9 {
        match seen {
            Some(s) => {
                if s.contains(&position) {
                    return 0;
                }
                s.insert(position);
                return 1;
            }
            None => return 1,
        }
    }
    DIRS.iter().fold(0, |acc, dir| {
        let p = Point {
            x: position.x + dir.dx,
            y: position.y + dir.dy,
        };
        acc + walk(grid, p, max_x, max_y, val, seen)
    })
}

fn get_start_positions(grid: &Vec<Vec<char>>) -> Vec<Point> {
    grid.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, row)| {
            let positions = row.iter().enumerate().fold(Vec::new(), |mut a, (x, ch)| {
                if *ch == '0' {
                    a.push(Point {
                        x: x as isize,
                        y: y as isize,
                    });
                }
                a
            });
            acc.extend(positions);
            acc
        })
}
