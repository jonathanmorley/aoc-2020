use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};
use num::rational::Ratio;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

fn coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, (x, y)) = separated_pair(
        map_res(digit1, str::parse),
        char(','),
        map_res(digit1, str::parse),
    )(input)?;
    Ok((input, Coordinate { x, y }))
}

struct VentLine(Coordinate, Coordinate);

impl VentLine {
    fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn delta(&self) -> (i64, i64) {
        (
            self.1.x as i64 - self.0.x as i64,
            self.1.y as i64 - self.0.y as i64,
        )
    }

    fn step(&self) -> (i64, i64) {
        let (delta_x, delta_y) = self.delta();

        if delta_y == 0 {
            match delta_x.cmp(&0) {
                Ordering::Greater => (1, 0),
                Ordering::Equal => (0, 0),
                Ordering::Less => (-1, 0),
            }
        } else {
            let ratio = Ratio::new(delta_x, delta_y);
            let (reduced_x, reduced_y) = (*ratio.numer(), *ratio.denom());
            let (x_step, y_step) = (reduced_x.abs(), reduced_y.abs());

            match (delta_x >= 0, delta_y >= 0) {
                (true, true) => (x_step, y_step),
                (true, false) => (x_step, -y_step),
                (false, true) => (-x_step, y_step),
                (false, false) => (-x_step, -y_step),
            }
        }
    }

    fn points(&self) -> Vec<Coordinate> {
        let (step_x, step_y) = self.step();
        let (delta_x, delta_y) = self.delta();

        let steps = if step_x != 0 {
            (delta_x / step_x).abs()
        } else if step_y != 0 {
            (delta_y / step_y).abs()
        } else {
            0
        };

        (0..=steps)
            .map(|step| Coordinate {
                x: (self.0.x as i64 + (step * step_x)) as u32,
                y: (self.0.y as i64 + (step * step_y)) as u32,
            })
            .collect()
    }
}

fn vent_line(input: &str) -> IResult<&str, VentLine> {
    let (input, (from, to)) = separated_pair(coordinate, tag(" -> "), coordinate)(input)?;
    Ok((input, VentLine(from, to)))
}

fn generator(input: &str) -> Vec<VentLine> {
    input.lines().map(|l| vent_line(l).unwrap().1).collect()
}

pub fn part1(input: &str) -> usize {
    let input = generator(input);
    let vent_lines: Vec<&VentLine> = input
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();

    let vents: Vec<_> = vent_lines.iter().flat_map(|line| line.points()).collect();

    vents
        .iter()
        .filter(|vent| vents.iter().filter(|all| all == vent).count() >= 2)
        .sorted()
        .dedup()
        .count()
}

pub fn part2(input: &str) -> usize {
    let input = generator(input);

    let vents: Vec<_> = input.iter().flat_map(|line| line.points()).collect();

    vents
        .iter()
        .filter(|vent| vents.iter().filter(|all| all == vent).count() >= 2)
        .sorted()
        .dedup()
        .count()
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 5);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 12);
    }
}
