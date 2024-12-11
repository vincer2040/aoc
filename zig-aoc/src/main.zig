const std = @import("std");
const myreader = @import("./myreader.zig");
const day = @import("./day3.zig");

pub fn main() !void {
    const input = try myreader.read_from_file("../test-prompts/day3p1");
    defer std.heap.page_allocator.free(input);
    const result = try day.part_one(input);
    std.debug.print("result: {}\n", .{result});
}
