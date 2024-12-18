use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input, 70, 70, 1024);
    print_grid(&grid);
    shortest_path(&grid, (0, 0), (70, 70), 70, 70)
}

pub fn part_two(input: &str) -> Option<&str> {
    let mut grid = vec![vec!['.'; 71]; 71];
    let pos = input.lines().position(|line| {
        let (x_str, y_str) = line.split_once(',').expect("invalid input");
        let x: usize = x_str.parse().expect("invalid input");
        let y: usize = y_str.parse().expect("invalid input");
        grid[y][x] = '#';
        shortest_path(&grid, (0, 0), (70, 70), 70, 70).is_none()
    });

    match pos {
        Some(p) => Some(input.lines().collect::<Vec<&str>>()[p]),
        None => None,
    }
}

fn shortest_path(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize), max_x: usize, max_y: usize) -> Option<u32> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    q.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), distance)) = q.pop_front() {
        if (x, y) == end {
            return Some(distance);
        }

        for &(dx, dy) in &directions {
            let nx = (x as isize) + dx;
            let ny = (y as isize) + dy;
            if nx < 0 || ny < 0 || nx > max_x as isize || ny > max_y as isize {
                continue;
            }
            if grid[ny as usize][nx as usize] == '#' {
                continue;
            }

            if visited.contains(&(nx as usize, ny as usize)) {
                continue;
            }

            q.push_back(((nx as usize, ny as usize), distance + 1));
            visited.insert((nx as usize, ny as usize));
        }
    }
    None
}

fn parse_input(input: &str, max_x: usize, max_y: usize, num_bytes: usize) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; max_x + 1]; max_y + 1];
    input.lines().take(num_bytes).for_each(|line| {
        let (x_str, y_str) = line.split_once(',').expect("invalid input");
        let x: usize = x_str.parse().expect("invalid input");
        let y: usize = y_str.parse().expect("invalid input");
        res[y][x] = '#';
    });
    return res;
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for ch in row {
            print!("{}", ch);
        }
        println!("");
    }
}
