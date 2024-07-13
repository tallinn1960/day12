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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_swift_sample() {
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
    fn test_part1_swift() {
        let input = include_str!("../input.txt");
        let result = p1(input);
        assert_eq!(result, 7221);
    }
    #[test]
    fn test_part2_swift_sample() {
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
    fn test_part2_swift() {
        let input = include_str!("../input.txt");
        let result = p2(input);
        assert_eq!(result, 7139671893722);
    }
}