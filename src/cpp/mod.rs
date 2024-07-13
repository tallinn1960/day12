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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_cpp_sample() {
        let input = "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1
    ";
        let result = p1(input);
        assert_eq!(result, 21);
    }
    
    #[test]
    fn test_part1_cpp() {
        let input = include_str!("../../input.txt");
        let result = p1(input);
        assert_eq!(result, 7221);
    }
    #[test]
    fn test_part2_cpp_sample() {
        let input = "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";
        let result = p2(input);
        assert_eq!(result, 525152);
    }
    
    #[test]
    fn test_part2_cpp() {
        let input = include_str!("../../input.txt");
        let result = p2(input);
        assert_eq!(result, 7139671893722);
    }
}
