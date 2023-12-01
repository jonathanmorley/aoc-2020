use std::num::ParseIntError;

use itertools::Itertools;

fn generator(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    input
        .split("\n\n")
        .map(|chunk| chunk.lines().map(str::parse).collect())
        .collect()
}

pub fn part1(input: &str) -> usize {
    generator(input)
        .unwrap()
        .into_iter()
        .map(|calories| calories.into_iter().sum::<u32>())
        .max()
        .unwrap() as usize
}

pub fn part2(input: &str) -> usize {
    generator(input)
        .unwrap()
        .into_iter()
        .map(|calories| calories.into_iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum::<u32>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 24_000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 45_000);
    }
}
