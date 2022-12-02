use std::str::FromStr;

use anyhow::{anyhow, Result};
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq)]
struct Vector(Direction, u32);

impl FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        if let (Some(direction), Some(magnitude)) = (split.next(), split.next()) {
            Ok(Vector(direction.parse()?, magnitude.parse()?))
        } else {
            Err(anyhow!("Unable to parse '{}' as a vector", s))
        }
    }
}

fn generator(input: &str) -> Result<Vec<Vector>> {
    input.lines().map(str::parse).collect()
}

struct Location {
    horizontal: i64,
    depth: i64,
}

pub fn part1(input: &str) -> i64 {
    let input = &generator(input).unwrap();

    let mut location = Location {
        horizontal: 0,
        depth: 0,
    };

    for Vector(direction, magnitude) in input {
        match direction {
            Direction::Forward => location.horizontal += *magnitude as i64,
            Direction::Up => location.depth -= *magnitude as i64,
            Direction::Down => location.depth += *magnitude as i64,
        }
    }

    location.horizontal * location.depth
}

struct OrientedLocation {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

pub fn part2(input: &str) -> i64 {
    let input = &generator(input).unwrap();

    let mut location = OrientedLocation {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for Vector(direction, magnitude) in input {
        match direction {
            Direction::Down => location.aim += *magnitude as i64,
            Direction::Up => location.aim -= *magnitude as i64,
            Direction::Forward => {
                location.horizontal += *magnitude as i64;
                location.depth += location.aim * (*magnitude as i64)
            }
        }
    }

    location.horizontal * location.depth
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 150);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 900);
    }
}
