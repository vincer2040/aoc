const std = @import("std");
const myreader = @import("./myreader.zig");
const day1 = @import("./day1.zig");

pub fn main() !void {
    const input = try myreader.read_from_file("../prompts/day1");
    defer std.heap.page_allocator.free(input);
    const result = try day1.part_two(input);
    std.debug.print("result: {}\n", .{result});
}
