use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Add,
};

pub fn part_one(input: &str) -> u32 {
    let grid = parse_input(input);
    let start = find_ch(&grid, 'S');
    let end = find_ch(&grid, 'E');
    let cost = dijkstra(&grid, start, end);
    return cost;
}

pub fn part_two(input: &str) -> u32 {
    let grid = parse_input(input);
    let start = find_ch(&grid, 'S');
    let end = find_ch(&grid, 'E');
    let tiles = dijkstra_tiles(&grid, start, end);
    return tiles;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    position: (usize, usize),
    direction: Direction,
    cost: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State2 {
    position: (usize, usize),
    direction: Direction,
    cost: u32,
    tiles: HashSet<(usize, usize)>,
}

fn dijkstra_tiles(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut prio = BinaryHeap::new();
    let mut dist_map: HashMap<(Direction, (usize, usize)), u32> = HashMap::new();
    let mut min_cost = u32::MAX;
    let mut min_tiles: Vec<HashSet<(usize, usize)>> = Vec::new();

    let mut first_tiles = HashSet::new();
    first_tiles.insert(start);

    prio.push(Reverse(State2 {
        position: start,
        direction: Direction::East,
        cost: 0,
        tiles: first_tiles,
    }));

    while let Some(Reverse(State2 {
        position,
        direction,
        cost,
        tiles,
    })) = prio.pop()
    {
        let (x, y) = position;
        if position == end {
            if cost == min_cost {
                min_tiles.push(tiles);
                continue;
            } else if cost < min_cost {
                min_tiles.clear();
                min_tiles.push(tiles);
                min_cost = cost;
                continue;
            }
        }

        match dist_map.get_mut(&(direction, (x, y))) {
            Some(c) => {
                if cost > *c {
                    continue;
                }
                *c = cost;
            }
            None => {
                dist_map.insert((direction, (x, y)), cost);
            }
        }

        let (next_x, next_y) = position + direction;

        if grid[next_y][next_x] != '#' {
            let mut next = tiles.clone();
            next.insert((next_x, next_y));
            prio.push(Reverse(State2 {
                position: (next_x, next_y),
                direction,
                cost: cost + 1,
                tiles: next,
            }));
        }

        let (right_x, right_y) = position + direction.turn_right();

        if grid[right_y][right_x] != '#' {
            let mut right = tiles.clone();
            right.insert((right_x, right_y));
            prio.push(Reverse(State2 {
                position: (right_x, right_y),
                direction: direction.turn_right(),
                cost: cost + 1001,
                tiles: right,
            }));
        }

        let (left_x, left_y) = position + direction.turn_left();
        if grid[left_y][left_x] != '#' {
            let mut left = tiles.clone();
            left.insert((left_x, left_y));
            prio.push(Reverse(State2 {
                position: (left_x, left_y),
                direction: direction.turn_left(),
                cost: cost + 1001,
                tiles: left,
            }));
        }
    }
    let mut set = HashSet::new();
    for tiles in min_tiles {
        set.extend(tiles);
    }
    return set.len() as u32;
}

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut prio = BinaryHeap::new();
    let mut dist_map: HashMap<(Direction, (usize, usize)), u32> = HashMap::new();
    let mut min_cost = u32::MAX;

    prio.push(Reverse(State {
        position: start,
        direction: Direction::East,
        cost: 0,
    }));

    while let Some(Reverse(State {
        position,
        direction,
        cost,
    })) = prio.pop()
    {
        let (x, y) = position;
        if position == end && cost < min_cost {
            min_cost = cost;
            continue;
        }

        match dist_map.get_mut(&(direction, (x, y))) {
            Some(c) => {
                if cost > *c {
                    continue;
                }
                *c = cost;
            }
            None => {
                dist_map.insert((direction, (x, y)), cost);
            }
        }
        let (next_x, next_y) = position + direction;

        if grid[next_y][next_x] != '#' {
            prio.push(Reverse(State {
                position: (next_x, next_y),
                direction,
                cost: cost + 1,
            }));
        }

        let (right_x, right_y) = position + direction.turn_right();
        if grid[right_y][right_x] != '#' {
            prio.push(Reverse(State {
                position: (right_x, right_y),
                direction: direction.turn_right(),
                cost: cost + 1001,
            }));
        }

        let (left_x, left_y) = position + direction.turn_left();
        if grid[left_y][left_x] != '#' {
            prio.push(Reverse(State {
                position: (left_x, left_y),
                direction: direction.turn_left(),
                cost: cost + 1001,
            }));
        }
    }

    return min_cost;
}

fn find_ch(grid: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == c {
                return (x, y);
            }
        }
    }
    unreachable!();
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().collect());
        return acc;
    })
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);
    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;
        match rhs {
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
        }
    }
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
        }
    }
}
