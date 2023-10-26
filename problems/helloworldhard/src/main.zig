const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().outStream();

    try stdout.print("Hello, World!\n", .{});
    try stdout.print("Hello, World!\r", .{});
    try stdout.print("Hello, World!\r\n", .{});
}
