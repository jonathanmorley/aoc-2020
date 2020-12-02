use std::str::FromStr;
use std::{collections::HashSet, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Result<HashSet<u32>, ParseIntError> {
    input.lines().map(FromStr::from_str).collect()
}

#[aoc(day1, part1)]
pub fn part_1(input: &HashSet<u32>) -> Option<u32> {
    for &x in input {
        if x < 2020 / 2 {
            if let Some(complement) = complement(x) {
                if input.contains(&complement) {
                    return Some(x * complement);
                }
            }
        }
    }

    None
}

#[aoc(day1, part2)]
pub fn part_2(input: &HashSet<u32>) -> Option<u32> {
    for &x in input {
        if x < 2020 / 3 {
            for &y in input {
                if y > 2020 / 3 {
                    if let Some(complement) = complement(x + y) {
                        if input.contains(&complement) {
                            return Some(x * y * complement);
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

    static SAMPLE: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn test_parse() {
        assert_eq!(parse(SAMPLE), Ok(hashset!(1721, 979, 366, 299, 675, 1456)));
    }

    #[test]
    fn test_part_1() {
        let parsed = parse(SAMPLE).unwrap();
        assert_eq!(part_1(&parsed), Some(514579));
    }

    #[test]
    fn test_part_2() {
        let parsed = parse(SAMPLE).unwrap();
        assert_eq!(part_2(&parsed), Some(241861950));
    }

    #[test]
    fn test_complement() {
        assert_eq!(complement(1), Some(2019));
        assert_eq!(complement(2021), None);
    }
}
