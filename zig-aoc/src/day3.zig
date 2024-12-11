const std = @import("std");

fn is_digit(ch: u8) bool {
    return "0" <= ch and ch <= '9';
}

pub fn part_one(input: []const u8) !u32 {
    std.debug.print("{s}\n", .{input});
    return 0;
}
