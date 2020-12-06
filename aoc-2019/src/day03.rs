use anyhow::{bail, Result};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Point(i64, i64);

#[derive(Debug, Hash, PartialEq)]
pub enum Vector {
    Right(u32),
    Left(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("R", d) => Ok(Vector::Right(d.parse()?)),
            ("L", d) => Ok(Vector::Left(d.parse()?)),
            ("U", d) => Ok(Vector::Up(d.parse()?)),
            ("D", d) => Ok(Vector::Down(d.parse()?)),
            (a, _) => bail!("Unexpected Direction {}", a),
        }
    }
}

impl Vector {
    fn points(&self, origin: &Point) -> Vec<Point> {
        match &self {
            Vector::Right(d) => (1..=*d)
                .map(|i| Point(origin.0 + i as i64, origin.1))
                .collect(),
            Vector::Left(d) => (1..=*d)
                .map(|i| Point(origin.0 - i as i64, origin.1))
                .collect(),
            Vector::Up(d) => (1..=*d)
                .map(|i| Point(origin.0, origin.1 + i as i64))
                .collect(),
            Vector::Down(d) => (1..=*d)
                .map(|i| Point(origin.0, origin.1 - i as i64))
                .collect(),
        }
    }
}

#[derive(Debug, Hash, PartialEq)]
pub struct Wire(Vec<Vector>);

impl FromStr for Wire {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.split(",").map(|x| x.parse()).collect::<Result<_>>()?;
        Ok(Wire(vec))
    }
}

impl Wire {
    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let mut current_point = Point(0, 0);

        for vector in &self.0 {
            let mut next_points = vector.points(&current_point);
            current_point = next_points.last().unwrap_or(&current_point).clone();
            points.append(&mut next_points);
        }

        points
    }

    fn intersections(&self, other: &Self) -> Vec<Point> {
        let mut intersections = Vec::new();
        
        for point in self.points() {
            for other_point in other.points() {
                if point == other_point {
                    intersections.push(point);
                }
            }
        }

        intersections
    }
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Result<Vec<Wire>> {
    input.lines().map(|line| line.parse()).collect()
}

#[aoc(day3, part1)]
pub fn part_1(input: &[Wire]) -> usize {
    if let Some(wire) = input.get(0) {
        dbg!(wire.points());
    }

    input.len()
}

#[aoc(day3, part2)]
pub fn part_2(input: &[Wire]) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::Vector::*;
    use super::*;

    static SAMPLE: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(
            parse(SAMPLE)?,
            vec![
                Wire(vec![
                    Right(75),
                    Down(30),
                    Right(83),
                    Up(83),
                    Left(12),
                    Down(49),
                    Right(71),
                    Up(7),
                    Left(72)
                ]),
                Wire(vec![
                    Up(62),
                    Right(66),
                    Up(55),
                    Right(34),
                    Down(71),
                    Right(55),
                    Down(58),
                    Right(83)
                ])
            ]
        );
        Ok(())
    }

    #[test]
    fn test_points() -> Result<()> {
        assert_eq!(
            "R8,U5,L5,D3".parse::<Wire>()?.points(),
            vec![
                Point(1, 0),
                Point(2, 0),
                Point(3, 0),
                Point(4, 0),
                Point(5, 0),
                Point(6, 0),
                Point(7, 0),
                Point(8, 0),
                Point(8, 1),
                Point(8, 2),
                Point(8, 3),
                Point(8, 4),
                Point(8, 5),
                Point(7, 5),
                Point(6, 5),
                Point(5, 5),
                Point(4, 5),
                Point(3, 5),
                Point(3, 4),
                Point(3, 3),
                Point(3, 2)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let parsed = parse(SAMPLE)?;
        assert_eq!(part_1(&parsed), 2);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let parsed = parse(SAMPLE)?;
        assert_eq!(part_2(&parsed), 2);
        Ok(())
    }
}
