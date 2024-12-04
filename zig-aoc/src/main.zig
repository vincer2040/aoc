const std = @import("std");
const myreader = @import("./myreader.zig");
const day2 = @import("./day2.zig");

pub fn main() !void {
    const input = try myreader.read_from_file("../prompts/day2");
    defer std.heap.page_allocator.free(input);
    const result = try day2.part_two(input);
    std.debug.print("result: {}\n", .{result});
}
