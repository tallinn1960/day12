extern crate link_cplusplus;

extern "C" {
    fn part1_c(input: *const u8, input_len: usize) -> usize;
    fn part2_c(input: *const u8, input_len: usize) -> usize;
}

pub fn p1(input: &str) -> usize {
    unsafe { part1_c(input.as_ptr(), input.len()) }
}

pub fn p2(input: &str) -> usize {
    unsafe { part2_c(input.as_ptr(), input.len()) }
}
