use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[aoc(day1, part1)]
pub fn part_1(input: &[String]) -> usize {
    input.len()
}

#[aoc(day1, part2)]
pub fn part_2(input: &[String]) -> usize {
  input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "\n";

    #[test]
    fn test_parse() {
        assert_eq!(parse(SAMPLE), vec![""]);
    }

    #[test]
    fn test_part_1() {
        let parsed = parse(SAMPLE);
        assert_eq!(part_1(&parsed), 1);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse(SAMPLE);
        assert_eq!(part_2(&parsed), 1);
    }
}
