// BTreeMap is slightly slower here
use std::str::FromStr;

use ahash::AHashMap;
use rayon::prelude::*;

pub fn p1(input: &str) -> u64 {
    parse(input)
        .map(|(l, plan)| count_part1(l, &plan) as u64)
        .sum::<u64>()
}

pub fn p2(input: &str) -> u64 {
    parse(input)
        .map(|(l, plan)| count_part2(l, &plan) as u64)
        .sum::<u64>()
}

fn parse<'a>(
    input: &'a str,
) -> rayon::iter::Map<
    rayon::str::Lines,
    impl Fn(&'a str) -> (&'a str, Vec<usize>),
> {
    input.par_lines().map(|line| {
        let mut parts = line.split(' ');
        let pattern = parts
            .next()
            .unwrap_or_else(|| panic!("No pattern in line {line}"));
        let plan = parts
            .next()
            .map(|p| {
                let numbers = p.split(',');
                numbers.fold(vec![], |mut acc, numberstring| {
                    acc.push(usize::from_str(numberstring).unwrap_or_else(
                        |_| panic!("Malformed number in line {line}"),
                    ));
                    acc
                })
            })
            .unwrap_or_else(|| panic!("No numbers in line {line}"));
        (pattern, plan)
    })
}
trait Cached<'a> {
    fn get(&self, key: &(&str, &[usize])) -> Option<usize>;
    fn insert(&mut self, key: (&'a str, &'a [usize]), value: usize);

    fn count(&mut self, cfg: &'a str, nums: &'a [usize]) -> usize {
        if cfg.is_empty() {
            if nums.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }
        if nums.is_empty() {
            if cfg.find('#').is_some() {
                return 0;
            } else {
                return 1;
            }
        }

        if let Some(result) = self.get(&(cfg, nums)) {
            return result;
        }

        let mut result = 0;
        let c = cfg.chars().next();

        if c == Some('.') || c == Some('?') {
            // ? is . case
            result += self.count(&cfg[1..], nums);
        }

        // if we treat the ? as # or have an #
        // we can start a block. Look for a sequence
        // of nums[0] characters that are # or ?
        // and terminated by a . or ?
        // that fulfills the first entry in nums,
        // handle the rest of the string with the
        // rest of the nums
        if (c == Some('#') || c == Some('?'))
            && nums[0] <= cfg.len()
            && cfg[..nums[0]].find('.').is_none()
            && (nums[0] == cfg.len() || cfg.chars().nth(nums[0]) != Some('#'))
        {
            // a block of nums[0] broken springs is possible
            // handle the rest, if any
            if nums[0] == cfg.len() {
                result += self.count("", &nums[1..])
            } else {
                result += self.count(&cfg[nums[0] + 1..], &nums[1..])
            }
        }
        self.insert((cfg, nums), result);
        result
    }
}

struct NoCache;

impl Cached<'_> for NoCache {
    fn get(&self, _key: &(&str, &[usize])) -> Option<usize> {
        None
    }

    fn insert(&mut self, _key: (&str, &[usize]), _value: usize) {}
}

fn count_part1(l: &str, plan: &[usize]) -> usize {
    let mut cache = NoCache {};
    cache.count(l, plan)
}

struct Cache<'a> {
    cache: AHashMap<(&'a str, &'a [usize]), usize>,
}

impl<'a> Cached<'a> for Cache<'a> {
    fn get(&self, key: &(&str, &[usize])) -> Option<usize> {
        self.cache.get(key).copied()
    }

    fn insert(&mut self, key: (&'a str, &'a [usize]), value: usize) {
        self.cache.insert(key, value);
    }
}

fn count_part2(cfg: &str, nums: &[usize]) -> usize {
    let newinput = std::iter::repeat(cfg).take(5).collect::<Vec<_>>().join("?");
    let newpattern = std::iter::repeat(nums)
        .take(5)
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    let mut cache = Cache {
        cache: AHashMap::new(),
    };
    cache.count(&newinput, &newpattern)
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
    fn test_part2() {
        let input = include_str!("../input.txt");
        let result = p2(input);
        assert_eq!(result, 7139671893722);
    }
}
