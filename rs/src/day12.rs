use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct IPoint {
    x: isize,
    y: isize,
}

struct Dir {
    dx: isize,
    dy: isize,
}

pub fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_y = grid.len() as isize;
    let max_x = grid[0].len() as isize;

    let mut seen: HashSet<Point> = HashSet::new();
    let mut plots: Vec<Vec<Point>> = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            let position = Point { x, y };
            if seen.contains(&position) {
                continue;
            }
            seen.insert(position);
            let mut s: HashSet<Point> = HashSet::new();
            let mut plants: Vec<Point> = Vec::new();

            plants = find_all_plants(
                &grid,
                position.into(),
                *ch,
                max_x,
                max_y,
                &mut s,
                &mut plants,
            );

            plots.push(plants);
            seen.extend(s);
        }
    }

    plots.iter().fold(0, |acc, plot| {
        let plant_char = grid[plot[0].y][plot[0].x];
        let area = plot.len() as u32;
        let perimeter = calculate_perimeter(&grid, &plot, plant_char, max_x, max_y);
        return acc + (area * perimeter);
    })
}

pub fn part_two(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_y = grid.len() as isize;
    let max_x = grid[0].len() as isize;

    let mut seen: HashSet<Point> = HashSet::new();
    let mut plots: Vec<Vec<Point>> = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            let position = Point { x, y };
            if seen.contains(&position) {
                continue;
            }
            seen.insert(position);
            let mut s: HashSet<Point> = HashSet::new();
            let mut plants: Vec<Point> = Vec::new();

            plants = find_all_plants(
                &grid,
                position.into(),
                *ch,
                max_x,
                max_y,
                &mut s,
                &mut plants,
            );

            plots.push(plants);
            seen.extend(s);
        }
    }

    plots.iter().fold(0, |acc, plot| {
        let plant_char = grid[plot[0].y][plot[0].x];
        let area = plot.len() as u32;
        let perimeter = calculate_perimeter2(&grid, plot, plant_char, max_x, max_y);
        return acc + (area * perimeter);
    })
}

const DIRS: [Dir; 4] = [
    Dir { dy: -1, dx: 0 },
    Dir { dy: 0, dx: 1 },
    Dir { dy: 1, dx: 0 },
    Dir { dy: 0, dx: -1 },
];

fn find_all_plants(
    grid: &Vec<Vec<char>>,
    position: IPoint,
    plant: char,
    max_x: isize,
    max_y: isize,
    seen: &mut HashSet<Point>,
    plants: &mut Vec<Point>,
) -> Vec<Point> {
    if position.x < 0 || position.x >= max_x || position.y < 0 || position.y >= max_y {
        return plants.to_vec();
    }
    if seen.contains(&position.into()) {
        return plants.to_vec();
    }

    if grid[position.y as usize][position.x as usize] != plant {
        return plants.to_vec();
    }

    seen.insert(position.into());
    plants.push(position.into());

    DIRS.iter().fold(plants.to_vec(), |mut acc, dir| {
        let p = IPoint {
            x: position.x + dir.dx,
            y: position.y + dir.dy,
        };
        return find_all_plants(grid, p, plant, max_x, max_y, seen, &mut acc);
    })
}

fn calculate_perimeter(
    grid: &Vec<Vec<char>>,
    plot: &Vec<Point>,
    plant_char: char,
    max_x: isize,
    max_y: isize,
) -> u32 {
    plot.iter().fold(0, |acc, plant| {
        let iplant: IPoint = (*plant).into();
        return acc
            + DIRS.iter().fold(0, |a, dir| {
                let ipoint = IPoint {
                    x: iplant.x + dir.dx,
                    y: iplant.y + dir.dy,
                };
                if ipoint.x < 0 || ipoint.x >= max_x || ipoint.y < 0 || ipoint.y >= max_y {
                    return a + 1;
                }
                let point: Point = ipoint.into();
                let check_plant_char = grid[point.y][point.x];
                if check_plant_char != plant_char {
                    return a + 1;
                }
                return a;
            });
    })
}

fn calculate_perimeter2(
    grid: &Vec<Vec<char>>,
    plot: &Vec<Point>,
    plant: char,
    max_x: isize,
    max_y: isize,
) -> u32 {
    DIRS.iter().fold(0, |acc, dir| {
        let mut edges = plot.iter().fold(Vec::new(), |mut a, point| {
            let mut ipoint: IPoint = (*point).into();
            ipoint.x += dir.dx;
            ipoint.y += dir.dy;
            if !ipoint.is_valid_point(max_x, max_y) {
                a.push(ipoint);
            } else {
                let p: Point = ipoint.into();
                if grid[p.y][p.x] != plant {
                    a.push(ipoint);
                }
            }
            return a;
        });
        let mut amt = 1;
        if dir.dx == 0 {
            edges.sort_by(|a, b| {
                if a.y == b.y {
                    a.x.cmp(&b.x)
                } else {
                    a.y.cmp(&b.y)
                }
            });

            edges.windows(2).for_each(|e| {
                let c = e[0];
                let n = e[1];
                if c.y != n.y {
                    amt += 1;
                } else {
                    if (n.x - c.x).abs() != 1 {
                        amt += 1;
                    }
                }
            });
        } else if dir.dy == 0 {
            edges.sort_by(|a, b| {
                if a.x == b.x {
                    a.y.cmp(&b.y)
                } else {
                    a.x.cmp(&b.x)
                }
            });

            edges.windows(2).for_each(|e| {
                let c = e[0];
                let n = e[1];
                if c.x != n.x {
                    amt += 1;
                } else {
                    if (n.y - c.y).abs() != 1 {
                        amt += 1;
                    }
                }
            });
        }
        return acc + amt;
    })
}

impl Into<Point> for IPoint {
    fn into(self) -> Point {
        assert!(self.x >= 0, "x: {} y: {}", self.x, self.y);
        assert!(self.y >= 0, "x: {} y: {}", self.x, self.y);
        Point {
            x: self.x as usize,
            y: self.y as usize,
        }
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
    pub fn is_valid_point(&self, max_x: isize, max_y: isize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < max_x && self.y < max_y
    }
}
