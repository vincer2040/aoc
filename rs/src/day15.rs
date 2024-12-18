use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Direction {
    dx: isize,
    dy: isize,
}

const UP: Direction = Direction { dx: 0, dy: -1 };
const RIGHT: Direction = Direction { dx: 1, dy: 0 };
const DOWN: Direction = Direction { dx: 0, dy: 1 };
const LEFT: Direction = Direction { dx: -1, dy: 0 };

pub fn part_one(input: &str) -> u32 {
    let (mut puzzle, directions) = parse_input(input, false);
    let mut point = find_robot(&puzzle);
    let max_x = puzzle[0].len();
    let max_y = puzzle.len();
    directions.iter().for_each(|dir| {
        point = move_direction(&mut puzzle, point, dir, max_x, max_y);
    });
    return puzzle.iter().enumerate().fold(0, |acc, (y, line)| {
        acc + line.iter().enumerate().fold(0, |a, (x, ch)| {
            if *ch != 'O' {
                return a;
            }
            return a + ((y as u32 * 100) + x as u32);
        })
    });
}

pub fn part_two(input: &str) -> u32 {
    let (mut puzzle, directions) = parse_input(input, true);
    let mut point = find_robot(&puzzle);
    let max_x = puzzle[0].len();
    let max_y = puzzle.len();
    directions.iter().for_each(|dir| {
        point = move_direction2(&mut puzzle, point, dir, max_x, max_y);
    });
    return puzzle.iter().enumerate().fold(0, |acc, (y, line)| {
        acc + line.iter().enumerate().fold(0, |a, (x, ch)| {
            if *ch != '[' {
                return a;
            }
            return a + ((y as u32 * 100) + x as u32);
        })
    });
}

fn move_direction2(
    puzzle: &mut Vec<Vec<char>>,
    point: Point,
    direction: &Direction,
    max_x: usize,
    max_y: usize,
) -> Point {
    let new_point = point.apply_direction(direction);
    if puzzle[new_point.y][new_point.x] == '.' {
        puzzle[new_point.y][new_point.x] = '@';
        puzzle[point.y][point.x] = '.';
        return new_point;
    }
    if puzzle[new_point.y][new_point.x] == '#' {
        return point;
    }

    if direction.dx == 1 {
        let mut i = new_point.x + 1;
        while i < max_x {
            if puzzle[new_point.y][i] == '#' {
                return point;
            }
            if puzzle[new_point.y][i] == '.' {
                puzzle[new_point.y][i] = ']';
                i -= 1;
                while i > new_point.x {
                    if puzzle[new_point.y][i] == '[' {
                        puzzle[new_point.y][i] = ']'
                    } else {
                        puzzle[new_point.y][i] = '['
                    }
                    i -= 1;
                }
                puzzle[point.y][point.x] = '.';
                puzzle[new_point.y][new_point.x] = '@';
                return new_point;
            }
            i += 1;
        }
        return point;
    }
    if direction.dx == -1 {
        let mut i = new_point.x - 1;
        while i > 0 {
            if puzzle[new_point.y][i] == '#' {
                return point;
            }
            if puzzle[new_point.y][i] == '.' {
                puzzle[new_point.y][i] = '[';
                i += 1;
                for k in i..new_point.x {
                    if puzzle[new_point.y][k] == '[' {
                        puzzle[new_point.y][k] = ']'
                    } else {
                        puzzle[new_point.y][k] = '['
                    }
                }
                puzzle[point.y][point.x] = '.';
                puzzle[new_point.y][new_point.x] = '@';
                return new_point;
            }
            i -= 1;
        }
    }

    if direction.dy == 1 {
        let (moving_parts, can_move) = get_connected(puzzle, 1, point, &mut HashSet::new());
        if !can_move {
            return point;
        }

        let mut y = max_y - 1;
        while y > new_point.y {
            let row = puzzle[y].clone();

            for x in 0..row.len() {
                if row[x] != '.' {
                    continue;
                }
                let to_check = Point { x, y: y - 1 };
                if !moving_parts.contains(&to_check) {
                    continue;
                }
                puzzle[y][x] = puzzle[to_check.y][to_check.x];
                puzzle[to_check.y][to_check.x] = '.';
            }
            y -= 1;
        }
        puzzle[new_point.y][new_point.x] = '@';
        puzzle[point.y][point.x] = '.';
        return new_point;
    }
    if direction.dy == -1 {
        let (moving_parts, can_move) = get_connected(puzzle, -1, point, &mut HashSet::new());
        if !can_move {
            return point;
        }
        let mut y = 0;
        while y < new_point.y {
            let row = puzzle[y].clone();

            for x in 0..row.len() {
                if row[x] != '.' {
                    continue;
                }
                let to_check = Point { x, y: y + 1 };
                if !moving_parts.contains(&to_check) {
                    continue;
                }
                puzzle[y][x] = puzzle[to_check.y][to_check.x];
                puzzle[to_check.y][to_check.x] = '.';
            }
            y += 1;
        }
        puzzle[new_point.y][new_point.x] = '@';
        puzzle[point.y][point.x] = '.';
        return new_point;
    }
    return point;
}

fn get_connected(
    puzzle: &Vec<Vec<char>>,
    dy: isize,
    point: Point,
    init: &mut HashSet<Point>,
) -> (HashSet<Point>, bool) {
    let new_point = point.apply_dy(dy);
    if puzzle[new_point.y][new_point.x] == '#' {
        return (init.clone(), false);
    }
    if puzzle[new_point.y][new_point.x] == '.' {
        return (init.clone(), true);
    }
    if init.contains(&new_point) {
        return (init.clone(), true);
    }
    init.insert(new_point);
    let mut can_move: bool;
    if puzzle[new_point.y][new_point.x] == '[' {
        (*init, can_move) = get_connected(puzzle, 0, new_point.apply_dx(1), init);
        if !can_move {
            return (init.clone(), can_move);
        }
        let next_point = point.apply_dy(dy);
        (*init, can_move) = get_connected(puzzle, dy, next_point, init);
        if !can_move {
            return (init.clone(), can_move);
        }
        (*init, can_move) = get_connected(puzzle, dy, next_point.apply_dx(1), init);
        if !can_move {
            return (init.clone(), can_move);
        }
    } else {
        assert_eq!(puzzle[new_point.y][new_point.x], ']');
        (*init, can_move) = get_connected(puzzle, 0, new_point.apply_dx(-1), init);
        if !can_move {
            return (init.clone(), can_move);
        }
        let next_point = point.apply_dy(dy);
        (*init, can_move) = get_connected(puzzle, dy, next_point, init);
        if !can_move {
            return (init.clone(), can_move);
        }
        (*init, can_move) = get_connected(puzzle, dy, next_point.apply_dx(-1), init);
        if !can_move {
            return (init.clone(), can_move);
        }
    }
    return (init.clone(), can_move);
}

fn move_direction(
    puzzle: &mut Vec<Vec<char>>,
    point: Point,
    direction: &Direction,
    max_x: usize,
    max_y: usize,
) -> Point {
    let new_point = point.apply_direction(direction);
    if puzzle[new_point.y][new_point.x] == '.' {
        puzzle[new_point.y][new_point.x] = '@';
        puzzle[point.y][point.x] = '.';
        return new_point;
    }
    if puzzle[new_point.y][new_point.x] == '#' {
        return point;
    }

    assert_eq!(puzzle[new_point.y][new_point.x], 'O');

    if direction.dy == -1 {
        let mut i = new_point.y - 1;
        while i > 0 {
            if puzzle[i][new_point.x] == '#' {
                return point;
            }
            if puzzle[i][new_point.x] == '.' {
                puzzle[i][new_point.x] = 'O';
                puzzle[new_point.y][new_point.x] = '@';
                puzzle[point.y][point.x] = '.';
                return new_point;
            }
            i -= 1;
        }
        return point;
    }

    if direction.dx == 1 {
        let mut i = new_point.x + 1;
        while i < max_x {
            if puzzle[new_point.y][i] == '#' {
                return point;
            }
            if puzzle[new_point.y][i] == '.' {
                puzzle[new_point.y][i] = 'O';
                puzzle[new_point.y][new_point.x] = '@';
                puzzle[point.y][point.x] = '.';
                return new_point;
            }
            i += 1;
        }
        return point;
    }

    if direction.dy == 1 {
        let mut i = new_point.y + 1;
        while i < max_y {
            if puzzle[i][new_point.x] == '#' {
                return point;
            }
            if puzzle[i][new_point.x] == '.' {
                puzzle[i][new_point.x] = 'O';
                puzzle[new_point.y][new_point.x] = '@';
                puzzle[point.y][point.x] = '.';
                return new_point;
            }
            i += 1;
        }
        return point;
    }

    if direction.dx == -1 {
        let mut i = new_point.x - 1;
        while i > 0 {
            if puzzle[new_point.y][i] == '#' {
                return point;
            }
            if puzzle[new_point.y][i] == '.' {
                puzzle[new_point.y][i] = 'O';
                puzzle[new_point.y][new_point.x] = '@';
                puzzle[point.y][point.x] = '.';
                return new_point;
            }
            i -= 1;
        }

        return point;
    }
    unreachable!();
}

fn find_robot(puzzle: &Vec<Vec<char>>) -> Point {
    for (y, row) in puzzle.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '@' {
                return Point { x, y };
            }
        }
    }
    unreachable!();
}

fn parse_input(input: &str, part_two: bool) -> (Vec<Vec<char>>, Vec<Direction>) {
    let (puzzle_str, direction_str) = input.trim().split_once("\n\n").expect("invalid input");
    let puzzle = puzzle_str.lines().fold(Vec::new(), |mut acc, line| {
        if !part_two {
            let row: Vec<char> = line.chars().collect();
            acc.push(row);
            return acc;
        }
        let mut row = Vec::new();
        for ch in line.chars() {
            if ch == '@' {
                row.push(ch);
                row.push('.');
            } else if ch == 'O' {
                row.push('[');
                row.push(']');
            } else {
                row.push(ch);
                row.push(ch);
            }
        }
        acc.push(row);
        return acc;
    });
    let directions = direction_str
        .replace("\n", "")
        .chars()
        .fold(Vec::new(), |mut acc, ch| {
            acc.push(Direction::from(ch));
            return acc;
        });
    return (puzzle, directions);
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => unreachable!(),
        }
    }
}

impl Point {
    fn apply_direction(&self, direction: &Direction) -> Self {
        Self {
            x: (self.x as isize + direction.dx) as usize,
            y: (self.y as isize + direction.dy) as usize,
        }
    }

    fn apply_dy(&self, dy: isize) -> Self {
        Self {
            x: self.x,
            y: (self.y as isize + dy) as usize,
        }
    }

    fn apply_dx(&self, dx: isize) -> Self {
        Self {
            x: (self.x as isize + dx) as usize,
            y: self.y,
        }
    }
}
