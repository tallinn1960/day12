pub mod cpp;

use std::str::FromStr;

// BTreeMap is slightly slower here
use ahash::AHashMap;
use rayon::prelude::*;

pub fn p1(input: &str) -> u64 {
    input
        .par_lines()
        .map(parse)
        .map(|(pattern, groups)| {
            // part1 is solved much faster without an cache
            count(&mut NoCache::default(), pattern, &groups) as u64
        })
        .sum::<u64>()
}

pub fn p1_single_threaded(input: &str) -> u64 {
    input
        .lines()
        .map(parse)
        .map(|(pattern, groups)| {
            // part1 is solved much faster without an cache
            count(&mut NoCache::default(), pattern, &groups) as u64
        })
        .sum::<u64>()
}

pub fn p2(input: &str) -> u64 {
    input
        .par_lines()
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

fn parse<'a>(line: &'a str) -> (&'a str, Vec<usize>) {
    let mut parts = line.split(' ');
    let pattern = parts
        .next()
        .unwrap_or_else(|| panic!("No pattern in line {line}"));
    let plan = parts
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
    (pattern, plan)
}

trait CacheStorage<'a>: Default {
    fn get(&self, key: &(&str, &[usize])) -> Option<usize>;
    fn insert(&mut self, key: (&'a str, &'a [usize]), value: usize);
}

fn count<'a>(
    cache: &mut impl CacheStorage<'a>,
    pattern: &'a str,
    groups: &'a [usize],
) -> usize {
    // uses recursion, but stack usage per call is low and depth is
    // limited by the length of the pattern
    if pattern.is_empty() {
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }
    if groups.is_empty() {
        if pattern.find('#').is_some() {
            return 0;
        } else {
            return 1;
        }
    }

    if let Some(result) = cache.get(&(pattern, groups)) {
        return result;
    }

    let mut result = 0;
    let c = pattern.chars().next();

    if c == Some('.') || c == Some('?') {
        // ? is . case
        result += count(cache, &pattern[1..], groups);
    }

    // if we treat the ? as # or have an #
    // we can start a block. Look for a sequence
    // of nums[0] characters that are # or ?
    // and terminated by a . or ?
    // that fulfills the first entry in nums,
    // handle the rest of the string with the
    // rest of the nums
    if (c == Some('#') || c == Some('?'))
        && groups[0] <= pattern.len()
        && pattern[..groups[0]].find('.').is_none()
        && (groups[0] == pattern.len()
            || pattern.chars().nth(groups[0]) != Some('#'))
    {
        // a block of nums[0] broken springs is possible
        // handle the rest, if any
        if groups[0] == pattern.len() {
            result += count(cache, "", &groups[1..])
        } else {
            result += count(cache, &pattern[groups[0] + 1..], &groups[1..])
        }
    }
    cache.insert((pattern, groups), result);
    result
}

#[derive(Default)]
struct NoCache;

impl CacheStorage<'_> for NoCache {
    fn get(&self, _key: &(&str, &[usize])) -> Option<usize> {
        None
    }

    fn insert(&mut self, _key: (&str, &[usize]), _value: usize) {}
}

#[derive(Default)]
struct Cache<'a> {
    cache: AHashMap<(&'a str, &'a [usize]), usize>,
}

impl<'a> CacheStorage<'a> for Cache<'a> {
    fn get(&self, key: &(&str, &[usize])) -> Option<usize> {
        self.cache.get(key).copied()
    }

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
    fn test_part1_cpp_sample() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = cpp::p1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2_sample() {
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
    fn test_part1() {
        let input = include_str!("../input.txt");
        let result = p1(input);
        assert_eq!(result, 7221);
    }

    #[test]
    fn test_part1_cpp() {
        let input = include_str!("../input.txt");
        let result = cpp::p1(input);
        assert_eq!(result, 7221);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        let result = p2(input);
        assert_eq!(result, 7139671893722);
    }
}
