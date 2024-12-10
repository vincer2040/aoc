use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum BlockItem {
    FreeSpace,
    File(u32),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum BlockItemSpan {
    FreeSpace(usize),
    File(u32, usize),
}

pub fn part_one(input: &str) -> u64 {
    let (mut blocks, _): (Vec<BlockItem>, u32) =
        input
            .trim()
            .chars()
            .enumerate()
            .fold((Vec::new(), 0), |(mut blocks, mut id), (i, ch)| {
                if i % 2 == 0 {
                    // file
                    let num = ch.to_digit(10).expect(&format!("{} not u32", ch));
                    for _ in 0..num {
                        blocks.push(BlockItem::File(id));
                    }
                    id += 1;
                } else {
                    // free space
                    let num = ch.to_digit(10).expect(&format!("{} not u32", ch));
                    for _ in 0..num {
                        blocks.push(BlockItem::FreeSpace);
                    }
                }
                (blocks, id)
            });
    let mut free_space_ptr = 0;
    let mut file_ptr: isize = (blocks.len() - 1) as isize;
    while free_space_ptr <= file_ptr as usize {
        let free_space = blocks[free_space_ptr];
        if free_space != BlockItem::FreeSpace {
            free_space_ptr += 1;
            continue;
        }
        let file = blocks[file_ptr as usize];
        match file {
            BlockItem::FreeSpace => {
                file_ptr -= 1;
                continue;
            }
            _ => {}
        }

        blocks[free_space_ptr] = file;
        blocks[file_ptr as usize] = free_space;
        free_space_ptr += 1;
        file_ptr -= 1;
    }

    blocks
        .iter()
        .filter(|x| **x != BlockItem::FreeSpace)
        .enumerate()
        .fold(0, |acc, (i, cur)| match cur {
            BlockItem::File(id) => acc + ((i * ((*id) as usize)) as u64),
            _ => acc,
        })
}

pub fn part_two(input: &str) -> u64 {
    let (mut blocks, _): (Vec<BlockItemSpan>, u32) =
        input
            .trim()
            .chars()
            .enumerate()
            .fold((Vec::new(), 0), |(mut blocks, mut id), (i, ch)| {
                if i % 2 == 0 {
                    // file
                    let num = ch.to_digit(10).expect(&format!("{} not u32", ch));
                    blocks.push(BlockItemSpan::File(id, num as usize));
                    id += 1;
                } else {
                    // free space
                    let num = ch.to_digit(10).expect(&format!("{} not u32", ch));
                    blocks.push(BlockItemSpan::FreeSpace(num as usize))
                }
                (blocks, id)
            });

    let mut file_ptr = blocks.len() - 1;

    while file_ptr > 0 {
        let file = blocks[file_ptr];
        let file_span: usize;
        match file {
            BlockItemSpan::File(_, span) => {
                file_span = span;
            }
            _ => {
                file_ptr -= 1;
                continue;
            }
        }

        let len = blocks.len();
        for free_space_ptr in 0..len {
            if free_space_ptr >= file_ptr {
                break;
            }
            let free_space = blocks[free_space_ptr];
            let free_space_span: usize;
            match free_space {
                BlockItemSpan::FreeSpace(span) => free_space_span = span,
                _ => continue,
            }
            if free_space_span >= file_span {
                let remaining_space = free_space_span - file_span;
                blocks[free_space_ptr] = file;
                blocks[file_ptr] = BlockItemSpan::FreeSpace(file_span);
                if remaining_space > 0 {
                    blocks.insert(
                        free_space_ptr + 1,
                        BlockItemSpan::FreeSpace(remaining_space),
                    );
                }
                break;
            }
        }
        file_ptr -= 1;
    }

    let mut splat_blocks = Vec::new();
    for item in blocks {
        match item {
            BlockItemSpan::FreeSpace(span) => {
                for _ in 0..span {
                    splat_blocks.push(BlockItem::FreeSpace);
                }
            }
            BlockItemSpan::File(id, span) => {
                for _ in 0..span {
                    splat_blocks.push(BlockItem::File(id));
                }
            }
        }
    }

    splat_blocks
        .iter()
        .enumerate()
        .fold(0, |acc, (i, cur)| match cur {
            BlockItem::File(id) => acc + ((i * ((*id) as usize)) as u64),
            _ => acc,
        })
}

impl Display for BlockItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            BlockItem::FreeSpace => write!(f, "."),
            BlockItem::File(id) => write!(f, "{}", id),
        };
        res
    }
}

impl Display for BlockItemSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockItemSpan::FreeSpace(span) => {
                for _ in 0..*span {
                    write!(f, ".")?;
                }
            }
            BlockItemSpan::File(id, span) => {
                for _ in 0..*span {
                    write!(f, "{}", id)?
                }
            }
        };
        Ok(())
    }
}
