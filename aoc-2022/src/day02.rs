fn generator(input: &str) -> Vec<(&str, &str)> {
    input.lines()
        .map(str::split_whitespace)
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .collect()
}

pub fn part1(input: &str) -> usize {
    generator(input)
        .into_iter()
        .map(|event| match event {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,
            _ => unreachable!()
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    generator(input)
        .into_iter()
        .map(|event| match event {
            ("A", "X") => 3 + 0,
            ("A", "Y") => 1 + 3,
            ("A", "Z") => 2 + 6,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 2 + 0,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 1 + 6,
            _ => unreachable!()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 12);
    }
}
