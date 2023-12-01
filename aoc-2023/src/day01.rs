fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &str) -> u32 {
    generator(input)
        .into_iter()
        .map(|line| {
            let first_digit = line
                .chars()
                .find(char::is_ascii_digit)
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last_digit = line
                .chars()
                .rfind(char::is_ascii_digit)
                .unwrap()
                .to_digit(10)
                .unwrap();

            (first_digit * 10) + last_digit
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    generator(input)
        .into_iter()
        .map(|line| {
            let needles = [
                ("0", 0),
                ("1", 1),
                ("one", 1),
                ("2", 2),
                ("two", 2),
                ("3", 3),
                ("three", 3),
                ("4", 4),
                ("four", 4),
                ("5", 5),
                ("five", 5),
                ("6", 6),
                ("six", 6),
                ("7", 7),
                ("seven", 7),
                ("8", 8),
                ("eight", 8),
                ("9", 9),
                ("nine", 9),
            ];

            let first_digit = needles
                .iter()
                .filter_map(|needle| line.find(needle.0).map(|idx| (idx, needle.1)))
                .min_by(|a, b| a.0.cmp(&b.0))
                .unwrap()
                .1;
            let last_digit = needles
                .iter()
                .filter_map(|needle| line.rfind(needle.0).map(|idx| (idx, needle.1)))
                .max_by(|a, b| a.0.cmp(&b.0))
                .unwrap()
                .1;

            dbg!(first_digit, last_digit);
            (first_digit * 10) + last_digit
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const SAMPLE_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    const SAMPLE_3: &str = "19581";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_2), 281);
        assert_eq!(part2(SAMPLE_3), 11);
    }
}
