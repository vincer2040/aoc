#[derive(Debug)]
struct Direction {
    pub dx: isize,
    pub dy: isize,
}

#[derive(Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Direction {
    pub fn get_point(
        &self,
        num_times: usize,
        cur_x: usize,
        cur_y: usize,
        max_x: usize,
        max_y: usize,
    ) -> Option<Point> {
        let x = (cur_x as isize) + (self.dx * (num_times as isize));
        let y = (cur_y as isize) + (self.dy * (num_times as isize));
        if x < 0 || x >= (max_x as isize) {
            return None;
        }
        if y < 0 || y >= (max_y as isize) {
            return None;
        }
        return Some(Point {
            x: x as usize,
            y: y as usize,
        });
    }
}

const DIRS: [Direction; 8] = [
    // north
    Direction { dx: 0, dy: -1 },
    // east
    Direction { dx: 1, dy: 0 },
    // south
    Direction { dx: 0, dy: 1 },
    // west
    Direction { dx: -1, dy: 0 },
    // north east
    Direction { dx: 1, dy: -1 },
    // south east
    Direction { dx: 1, dy: 1 },
    // south west
    Direction { dx: -1, dy: 1 },
    // north west
    Direction { dx: -1, dy: -1 },
];

const XDIRS: [(Direction, Direction); 2] = [
    // north east -> south west
    (Direction { dx: 1, dy: -1 }, Direction { dx: -1, dy: 1 }),
    // north west -> south east
    (Direction { dx: -1, dy: -1 }, Direction { dx: 1, dy: 1 }),
];

const NEXT_LETTERS: [char; 3] = ['M', 'A', 'S'];

pub fn part_one(input: &str) -> u32 {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let max_y = lines.len();

    lines.iter().enumerate().fold(0, |acc, (i, line)| {
        let max_x = line.len();
        acc + line.chars().enumerate().fold(0, |a, (j, ch)| {
            if ch != 'X' {
                return a;
            }
            return a + walk(&lines, j, i, max_x, max_y);
        })
    })
}

pub fn part_two(input: &str) -> u32 {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let max_y = lines.len();

    lines.iter().enumerate().fold(0, |acc, (i, line)| {
        let max_x = line.len();
        acc + line.chars().enumerate().fold(0, |a, (j, ch)| {
            if ch != 'A' {
                return a;
            }
            return a + xwalk(&lines, j, i, max_x, max_y);
        })
    })
}

fn walk(lines: &Vec<&str>, x: usize, y: usize, max_x: usize, max_y: usize) -> u32 {
    DIRS.iter().fold(0, |acc, dir| {
        let amt = NEXT_LETTERS.iter().enumerate().fold(0, |a, (i, ch)| {
            let point = match dir.get_point(i + 1, x, y, max_x, max_y) {
                Some(p) => p,
                None => return a,
            };
            if lines[point.y].as_bytes()[point.x] != (*ch as u8) {
                return a;
            }
            return a + 1;
        });
        if amt != 3 {
            return acc;
        }
        return acc + 1;
    })
}

fn xwalk(lines: &Vec<&str>, x: usize, y: usize, max_x: usize, max_y: usize) -> u32 {
    let x = XDIRS.iter().fold(0, |acc, dir| {
        let point_1 = match dir.0.get_point(1, x, y, max_x, max_y) {
            Some(p) => p,
            None => return acc,
        };
        let point_2 = match dir.1.get_point(1, x, y, max_x, max_y) {
            Some(p) => p,
            None => return acc,
        };
        let ch1 = lines[point_1.y].as_bytes()[point_1.x] as char;
        let exp_ch = match get_expected_char(ch1) {
            Some(ch) => ch,
            None => return acc,
        };
        let ch2 = lines[point_2.y].as_bytes()[point_2.x] as char;
        if ch2 != exp_ch {
            return acc;
        }
        return acc + 1;
    });
    if x != 2 {
        return 0;
    }
    return 1;
}

fn get_expected_char(cur_char: char) -> Option<char> {
    match cur_char {
        'M' => Some('S'),
        'S' => Some('M'),
        _ => None,
    }
}
