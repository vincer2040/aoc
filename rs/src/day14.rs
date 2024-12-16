use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct IPoint {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Velocity {
    dx: isize,
    dy: isize,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Velocity,
}

pub fn part_one(input: &str) -> u32 {
    let robots = parse_input(input);
    let max_x = 101;
    let max_y = 103;

    let mid_x = max_x / 2;
    let mid_y = max_y / 2;

    let mut grid = create_grid(max_x, max_y);
    insert_robots(&mut grid, &robots);

    for _ in 0..100 {
        grid = blink(&grid, max_x as isize, max_y as isize);
    }

    let q1 = calculate_quadrant(&grid, 0, mid_x, 0, mid_y);
    let q2 = calculate_quadrant(&grid, mid_x + 1, max_x, 0, mid_y);
    let q3 = calculate_quadrant(&grid, 0, mid_x, mid_y + 1, max_y);
    let q4 = calculate_quadrant(&grid, mid_x + 1, max_x, mid_y + 1, max_y);
    return q1 * q2 * q3 * q4;
}

pub fn part_two(input: &str) -> u32 {
    let robots = parse_input(input);
    let max_x = 101;
    let max_y = 103;

    let mut grid = create_grid(max_x, max_y);
    insert_robots(&mut grid, &robots);

    for i in 0..(max_x * max_y) {
        grid = blink(&grid, max_x as isize, max_y as isize);
        if check_for_line(&grid) {
            return (i + 1) as u32;
        }
    }
    return 0;
}

fn check_for_line(grid: &Vec<Vec<Option<Vec<Velocity>>>>) -> bool {
    for row in grid {
        for robots in row.windows(31) {
            if !robots.contains(&None) {
                return true;
            }
        }
    }
    return false;
}

fn calculate_quadrant(
    grid: &Vec<Vec<Option<Vec<Velocity>>>>,
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
) -> u32 {
    let mut res = 0;
    for y in start_y..end_y {
        for x in start_x..end_x {
            match &grid[y][x] {
                Some(robots) => res += robots.len() as u32,
                None => {}
            }
        }
    }
    return res;
}

fn blink(
    grid: &Vec<Vec<Option<Vec<Velocity>>>>,
    max_x: isize,
    max_y: isize,
) -> Vec<Vec<Option<Vec<Velocity>>>> {
    let mut new_grid = create_grid(max_x as usize, max_y as usize);
    for (y, row) in grid.iter().enumerate() {
        for (x, robots) in row.iter().enumerate() {
            match robots {
                Some(robots) => {
                    let ipoint: IPoint = Point { x, y }.into();
                    for robot in robots {
                        let mut new_ipoint = IPoint {
                            x: ipoint.x + robot.dx,
                            y: ipoint.y + robot.dy,
                        };
                        new_ipoint.normalize(max_x, max_y);
                        let new_point: Point = new_ipoint.into();
                        match new_grid
                            .get_mut(new_point.y)
                            .expect("invalid data")
                            .get_mut(new_point.x)
                            .expect("invalid input")
                        {
                            Some(v) => v.push(*robot),
                            None => new_grid[new_point.y][new_point.x] = Some(vec![*robot]),
                        }
                    }
                }
                None => {}
            }
        }
    }
    return new_grid;
}

fn insert_robots(grid: &mut Vec<Vec<Option<Vec<Velocity>>>>, robots: &Vec<Robot>) {
    robots.iter().for_each(|robot| {
        let pos = robot.position;
        match grid
            .get_mut(pos.y)
            .expect("invalid data")
            .get_mut(pos.x)
            .expect("invalid data")
        {
            Some(v) => v.push(robot.velocity),
            None => grid[pos.y][pos.x] = Some(vec![robot.velocity]),
        }
    });
}

fn create_grid(max_x: usize, max_y: usize) -> Vec<Vec<Option<Vec<Velocity>>>> {
    let mut res = Vec::new();
    res.reserve(max_y);
    for _ in 0..max_y {
        let mut row = Vec::new();
        row.reserve(max_x);
        for _ in 0..max_x {
            row.push(None);
        }
        res.push(row);
    }
    return res;
}

fn parse_input(input: &str) -> Vec<Robot> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        let (pos_dec, v_dec) = line.split_once(' ').expect("invalid input");
        let (_, pos_str) = pos_dec.split_once('=').expect("invalid input");
        let (x_str, y_str) = pos_str.split_once(',').expect("invalid input");
        let (_, v_str) = v_dec.split_once('=').expect("invalid input");
        let (dx_str, dy_str) = v_str.split_once(',').expect("invalid input");

        let start_position = Point {
            x: x_str.parse().expect("invalid input"),
            y: y_str.parse().expect("invalid input"),
        };
        let velocity = Velocity {
            dx: dx_str.parse().expect("invalid input"),
            dy: dy_str.parse().expect("invalid input"),
        };
        acc.push(Robot {
            position: start_position,
            velocity,
        });
        return acc;
    })
}

fn print_grid(grid: &Vec<Vec<Option<Vec<Velocity>>>>) {
    for row in grid {
        for robot in row {
            match robot {
                Some(r) => print!("{} ", r.len()),
                None => print!("0 "),
            }
        }
        println!("");
    }
}

impl Into<Point> for IPoint {
    fn into(self) -> Point {
        assert!(self.x >= 0);
        assert!(self.y >= 0);
        return Point {
            x: self.x as usize,
            y: self.y as usize,
        };
    }
}

impl Into<IPoint> for Point {
    fn into(self) -> IPoint {
        IPoint {
            x: self.x as isize,
            y: self.y as isize,
        }
    }
}

impl IPoint {
    pub fn normalize(&mut self, max_x: isize, max_y: isize) {
        if self.x < 0 {
            let diff = self.x.abs();
            self.x = max_x - diff;
        }
        if self.x >= max_x {
            let diff = self.x - max_x;
            self.x = diff;
        }
        if self.y < 0 {
            let diff = self.y.abs();
            self.y = max_y - diff;
        }
        if self.y >= max_y {
            let diff = self.y - max_y;
            self.y = diff;
        }
    }
}
