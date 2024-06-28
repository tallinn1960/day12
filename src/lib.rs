use std::str::FromStr;

#[allow(unused_variables)]
pub fn p1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let line = parts.next().unwrap();
            let plan = parts
                .next()
                .map(|p| {
                    let mut numbers = p.split(',');
                    let mut result = vec![];
                    while let Some(numberstring) = numbers.next() {
                        result.push(i32::from_str(numberstring).unwrap());
                    }
                    result
                })
                .unwrap();
            (line, plan)
        })
        .map(|(l, plan)| {
            generate_filtered(l, &plan)
                .map(to_consecutive_groups)
                .filter(|g| *g == plan)
                .count() as u64
        })
        .sum::<u64>()
}

#[allow(unused_variables)]
pub fn p2(input: &str) -> u64 {
    todo!()
}

fn matches_group(springs: &str, plan: &[i32]) -> bool {
    let mut plan = plan.iter();
    let mut group = plan.next().unwrap();
    let mut chars = springs.chars();
    while let Some(c) = chars.next() {
        if c == '#' {
            let mut count = 1;
            while let Some(c) = chars.next() {
                if c == '.' {
                    if count == *group {
                        let maybe_next_group = plan.next();
                        if maybe_next_group.is_some() {
                            group = maybe_next_group.unwrap();
                            break;
                        }
                    } else {
                        return false;
                    }
                } else {
                    count += 1;
                    if count > *group {
                        return false;
                    }
                };
            }
        }
    }
    return true;
}

fn generate_filtered(
    input: &str,
    plan: &[i32],
) -> impl Iterator<Item = String> {
    let mut result = vec!["".to_string()];

    for c in input.chars() {
        let mut new_stubs: Vec<String> = vec![];
        for mut s in result {
            if c == '?' {
                let mut copy = s.clone();
                copy.push('.');
                if matches_group(&copy, plan) {
                    new_stubs.push(copy);
                } // push this only if it matches group order
                s.push('#');
                if matches_group(&s, plan) {
                    new_stubs.push(s);
                }
            } else {
                s.push(c);
                if matches_group(&s, plan) {
                    new_stubs.push(s);
                }
            }
        }
        result = new_stubs;
    }
    result.into_iter()
}

fn to_consecutive_groups(s: String) -> Vec<i32> {
    let mut result = vec![];
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '#' {
            let mut counter = 1;
            while let Some(c) = chars.next() {
                if c != '#' {
                    break;
                }
                counter += 1;
            }
            result.push(counter);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use assertables::*;

    use super::*;

    #[test]
    fn test_map_to_groups() {
        let input = vec![
            "....###", "#...###", ".#..###", "##..###", "..#.###", "#.#.###",
            ".##.###", "###.###",
        ]
        .into_iter()
        .map(|s| s.to_string());
        let result = input.map(to_consecutive_groups).collect::<Vec<_>>();
        let expected = vec![
            vec![3],
            vec![1, 3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 3],
            vec![1, 1, 3],
            vec![2, 3],
            vec![3, 3],
        ];
        assert_set_eq!(result, expected);
    }

    #[test]
    fn test_line() {
        let plan = vec![1, 1, 3];
        let result = generate_filtered("???.###", &plan)
            .map(to_consecutive_groups)
            .filter(|g| *g == plan)
            .count();
        assert_eq!(result, 1);
        let plan = vec![1, 1, 3];
        let result = generate_filtered(".??..??...?##.", &plan)
            .map(to_consecutive_groups)
            .filter(|g| *g == plan)
            .count();
        assert_eq!(result, 4);
        let plan = vec![3, 2, 1];
        let result = generate_filtered("?###????????", &plan)
            .map(to_consecutive_groups)
            .filter(|g| *g == plan)
            .count();
        assert_eq!(result, 10);
    }

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
}
