use std::convert::TryInto;

use aoc_runner_derive::{aoc, aoc_generator};
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, PartialEq, Recap)]
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
  number_1: u32,
  number_2: u32,
  character: char,
  password: String
}

impl PasswordValidator {
  fn valid_by_count(&self) -> bool {
    let count = self.password.matches(self.character).count();
    count >= self.number_1.try_into().unwrap() && count <= self.number_2.try_into().unwrap()
  }

  fn valid_by_index(&self) -> bool {
    let index_1: usize = (self.number_1 - 1).try_into().unwrap();
    let index_2: usize = (self.number_2 - 1).try_into().unwrap();

    let valid_1 = self.password.char_indices().nth(index_1) == Some((index_1, self.character));
    let valid_2 = self.password.char_indices().nth(index_2) == Some((index_2, self.character));
        
    valid_1 ^ valid_2
  }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<PasswordValidator> {
    input
        .lines()
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse())
        .map(|x| x.unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part_1(input: &[PasswordValidator]) -> usize {
    input
      .into_iter()
      .filter(|&x| x.valid_by_count())
      .count()
}

#[aoc(day2, part2)]
pub fn part_2(input: &[PasswordValidator]) -> usize {
    input
      .into_iter()
      .filter(|&x| x.valid_by_index())
      .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;

    #[test]
    fn test_parse() {
        assert_eq!(parse(SAMPLE), vec!(
          PasswordValidator { number_1: 1, number_2: 3, character: 'a', password: "abcde".into() },
          PasswordValidator { number_1: 1, number_2: 3, character: 'b', password: "cdefg".into() },
          PasswordValidator { number_1: 2, number_2: 9, character: 'c', password: "ccccccccc".into() }
        ));
    }

    #[test]
    fn test_part_1() {
        let answer = part_1(&parse(SAMPLE));
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(&parse(SAMPLE));
        assert_eq!(answer, 1);
    }
}
