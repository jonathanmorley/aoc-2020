use std::ops::RangeInclusive;

pub fn parse(input: &str) -> Vec<(RangeInclusive<u8>, RangeInclusive<u8>)> {
    input
        .lines()
        .map(|line| {
            let elves = line.split_once(',').unwrap();
            let left = elves.0.split_once('-').unwrap();
            let right = elves.1.split_once('-').unwrap();
            (
                u8::from_str_radix(left.0, 10).unwrap()..=u8::from_str_radix(left.1, 10).unwrap(),
                u8::from_str_radix(right.0, 10).unwrap()..=u8::from_str_radix(right.1, 10).unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &[(RangeInclusive<u8>, RangeInclusive<u8>)]) -> usize {
    input
        .iter()
        .filter(|(left, right)| {
            left.start() <= right.start() && left.end() >= right.end()
                || right.start() <= left.start() && right.end() >= left.end()
        })
        .count()
}

pub fn part2(input: &[(RangeInclusive<u8>, RangeInclusive<u8>)]) -> usize {
    input
        .iter()
        .filter(|(left, right)| {
            left.contains(right.start())
                || left.contains(right.end())
                || right.contains(left.start())
                || right.contains(left.end())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(SAMPLE)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(SAMPLE)), 4);
    }
}
