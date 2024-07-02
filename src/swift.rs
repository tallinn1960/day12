extern "C" {
    fn part1_swift(input: *const u8, input_len: usize) -> i64;
    fn part2_swift(input: *const u8, input_len: usize) -> i64;
}

pub fn p1(input: &str) -> i64 {
    unsafe { part1_swift(input.as_ptr(), input.len()) }
}

pub fn p2(input: &str) -> i64 {
    unsafe { part2_swift(input.as_ptr(), input.len()) }
}
