#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start_position = get_start_position(&grid);
    return walk(
        &grid,
        start_position,
        Dir::Up,
        &mut std::collections::HashSet::new(),
        grid[0].len(),
        grid.len(),
    );
}

pub fn part_two(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start_position = get_start_position(&grid);
    return walk_two(
        &grid,
        start_position,
        Dir::Up,
        grid.len(),
        grid[0].len(),
        &mut std::collections::HashSet::new(),
        &start_position,
        &mut 0,
    );
}

fn walk_two(
    grid: &Vec<Vec<char>>,
    position: Point,
    dir: Dir,
    max_y: usize,
    max_x: usize,
    seen: &mut std::collections::HashSet<Point>,
    start_position: &Point,
    num_obs: &mut u32,
) -> u32 {
    if !is_in_bounds(&position, max_x, max_y) {
        return *num_obs;
    }
    if grid[position.y as usize][position.x as usize] == '#' {
        let (next_position, next_dir) = position.next_point_change_dir(dir);
        return walk_two(
            grid,
            next_position,
            next_dir,
            max_y,
            max_x,
            seen,
            start_position,
            num_obs,
        );
    }
    let point_to_check = position.point_to_check(dir, grid, max_x, max_y);
    match point_to_check {
        Some(p) if !seen.contains(&p) && p != *start_position => {
            let mut grid_clone = grid.clone();
            let mut line = grid_clone[p.y as usize].clone();
            line[p.x as usize] = '#';
            grid_clone[p.y as usize] = line;
            if check_for_cycle(
                &grid_clone,
                position,
                dir,
                max_x,
                max_y,
                &mut std::collections::HashSet::new(),
            ) {
                *num_obs += 1;
            }
        }
        _ => {}
    }
    seen.insert(position);
    let next_position = position.next_point(dir);
    return walk_two(
        grid,
        next_position,
        dir,
        max_y,
        max_x,
        seen,
        start_position,
        num_obs,
    );
}

fn check_for_cycle(
    grid: &Vec<Vec<char>>,
    position: Point,
    dir: Dir,
    max_x: usize,
    max_y: usize,
    seen: &mut std::collections::HashSet<(Point, Dir)>,
) -> bool {
    if !is_in_bounds(&position, max_x, max_y) {
        return false;
    }
    if seen.contains(&(position, dir)) {
        return true;
    }
    if grid[position.y as usize][position.x as usize] == '#' {
        let (next_position, next_dir) = position.next_point_change_dir(dir);
        return check_for_cycle(grid, next_position, next_dir, max_x, max_y, seen);
    }
    seen.insert((position, dir));
    let next_position = position.next_point(dir);
    return check_for_cycle(grid, next_position, dir, max_x, max_y, seen);
}

fn walk(
    grid: &Vec<Vec<char>>,
    position: Point,
    dir: Dir,
    seen: &mut std::collections::HashSet<Point>,
    max_x: usize,
    max_y: usize,
) -> u32 {
    if !is_in_bounds(&position, max_x, max_y) {
        return seen.len() as u32;
    }
    if grid[position.y as usize][position.x as usize] == '#' {
        let (new_position, new_dir) = position.next_point_change_dir(dir);
        return walk(grid, new_position, new_dir, seen, max_x, max_y);
    }
    seen.insert(position);
    let new_position = position.next_point(dir);
    return walk(grid, new_position, dir, seen, max_x, max_y);
}

fn get_start_position(grid: &Vec<Vec<char>>) -> Point {
    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == '^' {
                return Point {
                    x: x as isize,
                    y: y as isize,
                };
            }
        }
    }
    unreachable!("invalid input, couldn't find start position");
}

fn is_in_bounds(position: &Point, max_x: usize, max_y: usize) -> bool {
    return !(position.x < 0
        || position.x >= max_x as isize
        || position.y < 0
        || position.y >= max_y as isize);
}

impl Point {
    fn next_point(self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Dir::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
            Dir::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Dir::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn next_point_change_dir(self, dir: Dir) -> (Self, Dir) {
        match dir {
            Dir::Up => (
                Point {
                    x: self.x,
                    y: self.y + 1,
                },
                Dir::Right,
            ),
            Dir::Right => (
                Point {
                    x: self.x - 1,
                    y: self.y,
                },
                Dir::Down,
            ),
            Dir::Down => (
                Point {
                    x: self.x,
                    y: self.y - 1,
                },
                Dir::Left,
            ),
            Dir::Left => (
                Point {
                    x: self.x + 1,
                    y: self.y,
                },
                Dir::Up,
            ),
        }
    }

    fn point_to_check(
        self,
        dir: Dir,
        grid: &Vec<Vec<char>>,
        max_x: usize,
        max_y: usize,
    ) -> Option<Self> {
        let p = self.next_point(dir);
        if is_in_bounds(&p, max_x, max_y) && grid[p.y as usize][p.x as usize] != '#' {
            return Some(p);
        }
        return None;
    }
}
