#[cfg(feature = "cpp")]
pub mod cpp;
#[cfg(feature = "swift")]
pub mod swift;
#[cfg(feature = "par")]
use rayon::prelude::*;
#[cfg(feature = "par")]
macro_rules! lines {
    ($input:ident) => {
        $input.par_lines()
    };
}
#[cfg(not(feature = "par"))]
macro_rules! lines {
    ($input:ident) => {
        $input.lines()
    };
}

use std::str::FromStr;

// BTreeMap is slightly slower here
use ahash::AHashMap;
type CacheMap<'a> = AHashMap<(&'a str, &'a [usize]), usize>;

pub fn p1(input: &str) -> u64 {
    lines!(input)
        .map(parse)
        .map(|(pattern, groups)| {
            // part1 is solved much faster without an cache
            count(&mut NoCache, pattern, &groups) as u64
        })
        .sum::<u64>()
}

pub fn p2(input: &str) -> u64 {
    lines!(input)
        .map(parse)
        .map(|(l, plan)| {
            let newpattern =
                std::iter::repeat(l).take(5).collect::<Vec<_>>().join("?");
            let newgroups = std::iter::repeat(plan)
                .take(5)
                .flatten()
                .collect::<Vec<_>>();
            (newpattern, newgroups)
        })
        .map(|(pattern, groups)| {
            // part2 is solved much faster with a cache, its ms vs hours
            count(&mut Cache::default(), &pattern, &groups) as u64
        })
        .sum::<u64>()
}

fn parse(line: &str) -> (&str, Vec<usize>) {
    let mut parts = line.split(' ');
    let pattern = parts
        .next()
        .unwrap_or_else(|| panic!("No pattern in line {line}"));
    let groups = parts
        .next()
        .map(|p| {
            p.split(',').fold(vec![], |mut acc, numberstring| {
                acc.push(usize::from_str(numberstring).unwrap_or_else(|_| {
                    panic!("Malformed number in line {line}")
                }));
                acc
            })
        })
        .unwrap_or_else(|| panic!("No numbers in line {line}"));
    (pattern, groups)
}

trait CacheStorage<'a> {
    fn get(&self, key: &(&str, &[usize])) -> Option<usize>;
    fn insert(&mut self, key: (&'a str, &'a [usize]), value: usize);
}

fn count<'a>(
    cache: &mut impl CacheStorage<'a>,
    pattern: &'a str,
    groups: &'a [usize],
) -> usize {
    match (pattern, groups) {
        ("", []) =>  1,
        ("", _) =>  0,
        (_, []) => if pattern.contains('#') { 0 } else { 1 },
        _ => if let Some(result) = cache.get(&(pattern, groups)) {
                result
            } else {
                let mut result = 0;
                let c = pattern.chars().next();
            
                if c == Some('.') || c == Some('?') { // '.', '?' is '.' case
                    result += count(cache, pattern[1..].trim_start_matches(|c| c == '.'), groups);
                }
            
                if (c == Some('#') || c == Some('?')) // '#', '?' is '#' case
                    // there are enough chars in the pattern
                    && pattern.len() >= groups[0] 
                    // no . within group
                    && !pattern[..groups[0]].contains('.') 
                    // no # after the end
                    && pattern.chars().nth(groups[0]) != Some('#')
                {
                    // found a block of nums[0] broken springs in the pattern
                    // handle the rest, if any
                    result += count(cache, &pattern[pattern.len().min(groups[0] + 1)..], &groups[1..])
                }
                cache.insert((pattern, groups), result);
                result
            }   
    }
}

struct NoCache;

impl CacheStorage<'_> for NoCache {
    fn get(&self, _key: &(&str, &[usize])) -> Option<usize> {
        None
    }

    fn insert(&mut self, _key: (&str, &[usize]), _value: usize) {}
}

#[derive(Default)]
struct Cache<'a> {
    cache: CacheMap<'a>,
}

impl<'a> CacheStorage<'a> for Cache<'a> {
    #[inline]
    fn get(&self, key: &(&str, &[usize])) -> Option<usize> {
        self.cache.get(key).copied()
    }

    #[inline]
    fn insert(&mut self, key: (&'a str, &'a [usize]), value: usize) {
        self.cache.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = p1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let result = p1(input);
        assert_eq!(result, 7221);
    }

    #[test]
    fn test_part2_sample() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = p2(input);
        assert_eq!(result, 525152);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        let result = p2(input);
        assert_eq!(result, 7139671893722);
    }
}
