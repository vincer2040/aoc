const std = @import("std");

fn is_digit(ch: u8) bool {
    return '0' <= ch and ch <= '9';
}

pub fn part_one(input: []const u8) !u32 {
    const allocator = std.heap.page_allocator;
    var lhs = std.ArrayList(u32).init(allocator);
    var rhs = std.ArrayList(u32).init(allocator);
    defer lhs.deinit();
    defer rhs.deinit();

    var in_lhs = true;
    var cur_num: u32 = 0;
    for (0..input.len) |i| {
        const ch = input[i];
        if (is_digit(ch)) {
            cur_num = (cur_num * 10) + (ch - '0');
        } else if (ch == ' ' and in_lhs) {
            try lhs.append(cur_num);
            cur_num = 0;
            in_lhs = false;
        } else if (ch == '\n') {
            try rhs.append(cur_num);
            cur_num = 0;
            in_lhs = true;
        }
    }

    std.mem.sort(u32, lhs.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, rhs.items, {}, comptime std.sort.asc(u32));

    var result: u32 = 0;
    for (0..lhs.items.len) |i| {
        const lhs_item = lhs.items[i];
        const rhs_item = rhs.items[i];

        if (lhs_item > rhs_item) {
            result += (lhs_item - rhs_item);
        } else if (lhs_item < rhs_item) {
            result += (rhs_item - lhs_item);
        }
    }

    return result;
}

pub fn part_two(input: []const u8) !u32 {
    const allocator = std.heap.page_allocator;
    var lhs = std.ArrayList(u32).init(allocator);
    var rhs = std.ArrayList(u32).init(allocator);
    defer lhs.deinit();
    defer rhs.deinit();

    var in_lhs = true;
    var cur_num: u32 = 0;
    for (0..input.len) |i| {
        const ch = input[i];
        if (is_digit(ch)) {
            cur_num = (cur_num * 10) + (ch - '0');
        } else if (ch == ' ' and in_lhs) {
            try lhs.append(cur_num);
            cur_num = 0;
            in_lhs = false;
        } else if (ch == '\n') {
            try rhs.append(cur_num);
            cur_num = 0;
            in_lhs = true;
        }
    }
    var result: u32 = 0;

    var map = std.AutoHashMap(u32, u32).init(allocator);
    defer map.deinit();

    for (lhs.items) |lhs_item| {
        const amt = map.get(lhs_item);
        if (amt) |val| {
            result += lhs_item * val;
        } else {
            var val: u32 = 0;
            for (rhs.items) |rhs_item| {
                if (rhs_item == lhs_item) {
                    val += 1;
                }
            }
            try map.put(lhs_item, val);
            result += lhs_item * val;
        }
    }

    return result;
}
