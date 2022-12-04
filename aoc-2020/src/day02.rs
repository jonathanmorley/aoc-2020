use std::str::FromStr;

use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Recap)]
#[recap(regex = r#"(?x)
    (?P<number_1>\d+)
    -
    (?P<number_2>\d+)
    \s+
    (?P<character>.)
    :\s+
    (?P<password>\S+)
  "#)]
pub struct PasswordValidator {
    number_1: usize,
    number_2: usize,
    character: char,
    password: String,
}

impl PasswordValidator {
    fn valid_by_count(&self) -> bool {
        let count = self.password.matches(self.character).count();
        count >= self.number_1 && count <= self.number_2
    }

    // This is 1-indexed
    fn has_char_at_index(&self, idx: usize) -> bool {
        char::from(self.password.as_bytes()[idx - 1]) == self.character
    }

    fn valid_by_index(&self) -> bool {
        self.has_char_at_index(self.number_1) ^ self.has_char_at_index(self.number_2)
    }
}

pub fn parse(input: &str) -> impl Iterator<Item = PasswordValidator> + '_ {
    input.lines().map(FromStr::from_str).map(|x| x.unwrap())
}

pub fn part_1(input: &str) -> usize {
    parse(input).filter(|x| x.valid_by_count()).count()
}

pub fn part_2(input: &str) -> usize {
    parse(input).filter(|x| x.valid_by_index()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(SAMPLE).collect::<Vec<_>>(),
            vec!(
                PasswordValidator {
                    number_1: 1,
                    number_2: 3,
                    character: 'a',
                    password: "abcde".into()
                },
                PasswordValidator {
                    number_1: 1,
                    number_2: 3,
                    character: 'b',
                    password: "cdefg".into()
                },
                PasswordValidator {
                    number_1: 2,
                    number_2: 9,
                    character: 'c',
                    password: "ccccccccc".into()
                }
            )
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE), 1);
    }
}
