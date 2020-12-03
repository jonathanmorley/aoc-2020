use std::str::FromStr;
use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Result<FnvHashSet<u32>, ParseIntError> {
    input.lines().map(FromStr::from_str).collect()
}

#[aoc(day1, part1)]
pub fn part_1(input: &FnvHashSet<u32>) -> Option<u32> {
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
pub fn part_2(input: &FnvHashSet<u32>) -> Option<u32> {
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

    static SAMPLE: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn test_parse() {
        let set = vec![1721u32, 979u32, 366u32, 299u32, 675u32, 1456u32].into_iter().collect();
        assert_eq!(parse(SAMPLE), Ok(set));
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
