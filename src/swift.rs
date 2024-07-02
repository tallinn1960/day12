extern "C" {
    fn part1_swift(input: *const u8, input_len: usize) -> i64;
}

pub fn p1(input: &str) -> i64 {
    unsafe { part1_swift(input.as_ptr(), input.len()) }
}
