use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;
use std::str::FromStr;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> HashSet<u32> {
    input
        .lines()
        .filter(|&s| !s.is_empty())
        .map(u32::from_str)
        .map(|x| x.unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part_1(input: &HashSet<u32>) -> Option<u32> {
    for &x in input {
        if x < 2020/2 {
            if let Some(complement) = complement(x) {
                if input.contains(&complement) {
                    return Some(x * complement)
                }
            }
        }
    }
    
    None
}

#[aoc(day1, part2)]
pub fn part_2(input: &HashSet<u32>) -> Option<u32> {
    for &x in input {
        if x < 2020/3 {
            for &y in input {
                if y > 2020/3 {
                    if let Some(complement) = complement(x + y) {
                        if input.contains(&complement) {
                            return Some(x * y * complement)
                        }
                    }
                }
            }
        }
    }
    
    None
}

fn complement(x: u32) -> Option<u32> {
    2020u32.checked_sub(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    use maplit::hashset;

    static SAMPLE: &str = r#"
1721
979
366
299
675
1456
"#;

    #[test]
    fn test_parse() {
        assert_eq!(parse(SAMPLE), hashset!(
            1721,
            979,
            366,
            299,
            675,
            1456
        ));
    }

    #[test]
    fn test_part_1() {
        let answer = part_1(&parse(SAMPLE));
        assert_eq!(answer, Some(514579));
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(&parse(SAMPLE));
        assert_eq!(answer, Some(241861950));
    }

    #[test]
    fn test_complement() {
        assert_eq!(complement(1), Some(2019));
        assert_eq!(complement(2021), None);
    }
}
