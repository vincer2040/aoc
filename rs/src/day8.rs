#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Vec2 {
    dx: isize,
    dy: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct AntinodePoint {
    x: isize,
    y: isize,
}

pub fn part_one(input: &str) -> u32 {
    let (max_x, max_y, map): (isize, isize, std::collections::HashMap<char, Vec<Point>>) =
        input.trim().lines().enumerate().fold(
            (0, 0, std::collections::HashMap::new()),
            |(_, mut my, mut m), (y, line)| {
                my += 1;
                line.chars().enumerate().for_each(|(x, ch)| {
                    if ch != '.' {
                        match m.get_mut(&ch) {
                            Some(val) => val.push(Point { x, y }),
                            None => _ = m.insert(ch, vec![Point { x, y }]),
                        }
                    }
                });
                (line.len() as isize, my, m)
            },
        );
    map.iter()
        .fold(
            std::collections::HashSet::<AntinodePoint>::new(),
            |mut acc, (_, points)| {
                for (i, from_point) in points.iter().enumerate() {
                    for (j, to_point) in points.iter().enumerate() {
                        if i == j {
                            continue;
                        }
                        let vec = Vec2::new(from_point, to_point);
                        let antinode = AntinodePoint::new(from_point, &vec);
                        if antinode.is_valid(max_x, max_y) {
                            acc.insert(antinode);
                        }
                    }
                }
                acc
            },
        )
        .len() as u32
}

pub fn part_two(input: &str) -> u32 {
    let (max_x, max_y, map): (isize, isize, std::collections::HashMap<char, Vec<Point>>) =
        input.trim().lines().enumerate().fold(
            (0, 0, std::collections::HashMap::new()),
            |(_, mut my, mut m), (y, line)| {
                my += 1;
                line.chars().enumerate().for_each(|(x, ch)| {
                    if ch != '.' {
                        match m.get_mut(&ch) {
                            Some(val) => val.push(Point { x, y }),
                            None => _ = m.insert(ch, vec![Point { x, y }]),
                        }
                    }
                });
                (line.len() as isize, my, m)
            },
        );
    map.iter()
        .fold(
            std::collections::HashSet::<AntinodePoint>::new(),
            |mut acc, (_, points)| {
                let mut cur_antinodes = std::collections::HashSet::<AntinodePoint>::new();
                for (i, from_point) in points.iter().enumerate() {
                    let mut added = false;
                    for (j, to_point) in points.iter().enumerate() {
                        if i == j {
                            continue;
                        }
                        let mut fpoint = from_point.clone();
                        let mut tpoint = to_point.clone();
                        loop {
                            let vec = Vec2::new(&fpoint, &tpoint);
                            let antinode = AntinodePoint::new(&fpoint, &vec);
                            if antinode.is_valid(max_x, max_y) {
                                added = true;
                                cur_antinodes.insert(antinode);
                            } else {
                                added = false;
                                break;
                            }
                            fpoint = antinode.try_into().expect("already valid");
                            let tpoint_res: Result<Point, ()> =
                                AntinodePoint::new(&tpoint, &vec).try_into();
                            match tpoint_res {
                                Ok(tp) => {
                                    if tp.y >= max_y as usize || tp.x >= max_x as usize {
                                        break;
                                    }
                                    tpoint = tp;
                                }
                                _ => break,
                            }
                        }
                    }
                    if added == added {
                        cur_antinodes.insert((*from_point).into());
                    }
                }
                acc.extend(cur_antinodes);
                acc
            },
        )
        .len() as u32
}

impl Vec2 {
    pub fn new(from: &Point, to: &Point) -> Self {
        return Self {
            dx: (to.x as isize) - (from.x as isize),
            dy: (to.y as isize) - (from.y as isize),
        };
    }
}

impl AntinodePoint {
    pub fn new(antena: &Point, vec: &Vec2) -> Self {
        return Self {
            x: (antena.x as isize) + (vec.dx * -1),
            y: (antena.y as isize) + (vec.dy * -1),
        };
    }

    pub fn is_valid(&self, max_x: isize, max_y: isize) -> bool {
        return self.x >= 0 && self.x < max_x && self.y >= 0 && self.y < max_y;
    }
}

impl Into<AntinodePoint> for Point {
    fn into(self) -> AntinodePoint {
        AntinodePoint {
            x: self.x as isize,
            y: self.y as isize,
        }
    }
}

impl TryInto<Point> for AntinodePoint {
    type Error = ();
    fn try_into(self) -> Result<Point, Self::Error> {
        if self.x < 0 || self.y < 0 {
            return Err(());
        }
        return Ok(Point {
            x: self.x as usize,
            y: self.y as usize,
        });
    }
}
