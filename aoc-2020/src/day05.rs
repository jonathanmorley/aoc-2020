use anyhow::Result;
use std::{str::FromStr, num::TryFromIntError};
use std::convert::TryFrom;
use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Hash, PartialEq)]
pub struct Seat(u16);

impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binary_id = s
            .chars()
            .map(|x| match x {
                'B' | 'R' => 1,
                _ => 0
            })
            .join("");

        Ok(Seat(u16::from_str_radix(&binary_id, 2)?))
    }
}

impl Seat {
    pub fn row(&self) -> Result<u8, TryFromIntError> {
        u8::try_from(self.0 / 8)
    }

    pub fn column(&self) -> Result<u8, TryFromIntError> {
        u8::try_from(self.0 % 8)
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Result<Vec<Seat>> {
    input.lines().map(Seat::from_str).collect()
}

#[aoc(day5, part1)]
pub fn part_1(input: &[Seat]) -> Option<u16> {
    input.into_iter().map(|s| s.0).max()
}

fn minimum(input: &[Seat]) -> Option<u16> {
    input.into_iter().map(|s| s.0).min()
}

#[aoc(day5, part2)]
pub fn part_2(input: &[Seat]) -> Option<u16> {
    if let Some(max) = part_1(input) {
        if let Some(min) = minimum(input) {
            for i in (min+1)..max {
                if let None = input.iter().find(|&s| s.0 == i) {
                    return Some(i);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";

    #[test]
    fn test_seat() -> Result<()> {
        assert_eq!("BFFFBBFRRR".parse::<Seat>()?, Seat(567));
        assert_eq!("FFFBBBFRRR".parse::<Seat>()?, Seat(119));
        assert_eq!("BBFFBBFRLL".parse::<Seat>()?, Seat(820));
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(parse(SAMPLE)?, vec![
            Seat(567),
            Seat(119),
            Seat(820)
        ]);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let parsed = parse(SAMPLE)?;
        assert_eq!(part_1(&parsed), Some(820));
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let parsed = parse(SAMPLE)?;
        assert_eq!(part_2(&parsed), Some(120));
        Ok(())
    }
}
