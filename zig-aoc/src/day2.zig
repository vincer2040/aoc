const std = @import("std");

fn is_digit(ch: u8) bool {
    return '0' <= ch and ch <= '9';
}

const LevelType = enum {
    Increasing,
    Decreasing,
    Unknown,
};

fn is_safe(list: []u32) bool {
    var level_type: LevelType = LevelType.Unknown;
    for (0..list.len - 1) |i| {
        const cur = list[i];
        const next = list[i + 1];
        switch (level_type) {
            .Unknown => {
                if (next > cur) {
                    const diff: u32 = next - cur;
                    if (diff > 3) {
                        return false;
                    }
                    level_type = LevelType.Increasing;
                } else if (next < cur) {
                    const diff: u32 = cur - next;
                    if (diff > 3) {
                        return false;
                    }
                    level_type = LevelType.Decreasing;
                } else {
                    return false;
                }
            },
            .Increasing => {
                if (cur >= next) {
                    return false;
                }
                const diff: u32 = next - cur;
                if (diff > 3) {
                    return false;
                }
            },
            .Decreasing => {
                if (next >= cur) {
                    return false;
                }
                const diff: u32 = cur - next;
                if (diff > 3) {
                    return false;
                }
            },
        }
    }
    return true;
}

pub fn part_one(input: []const u8) !u32 {
    var result: u32 = 0;

    const allocator = std.heap.page_allocator;
    var level = std.ArrayList(u32).init(allocator);
    defer level.deinit();

    var cur_num: u32 = 0;
    for (0..input.len) |i| {
        const ch = input[i];
        if (ch == '\n') {
            try level.append(cur_num);
            const safe = is_safe(level.items);
            level.items.len = 0;
            cur_num = 0;
            if (safe) {
                result += 1;
            }
            continue;
        }
        if (!is_digit(ch)) {
            try level.append(cur_num);
            cur_num = 0;
            continue;
        }
        cur_num = (cur_num * 10) + (ch - '0');
    }
    return result;
}

pub fn part_two(input: []const u8) !u32 {
    var result: u32 = 0;

    const allocator = std.heap.page_allocator;
    var level = std.ArrayList(u32).init(allocator);
    defer level.deinit();

    var cur_num: u32 = 0;
    for (0..input.len) |i| {
        const ch = input[i];
        if (ch == '\n') {
            try level.append(cur_num);
            var safe = is_safe(level.items);
            if (safe) {
                result += 1;
            } else {
                var old: u32 = 0;
                for (0..level.items.len) |j| {
                    if (j != 0) {
                        try level.insert(j - 1, old);
                    }
                    old = level.orderedRemove(j);
                    safe = is_safe(level.items);
                    if (safe) {
                        result += 1;
                        break;
                    }
                }
            }
            level.items.len = 0;
            cur_num = 0;
            continue;
        }
        if (!is_digit(ch)) {
            try level.append(cur_num);
            cur_num = 0;
            continue;
        }
        cur_num = (cur_num * 10) + (ch - '0');
    }
    return result;
}
