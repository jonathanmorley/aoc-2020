use std::{cmp::Ordering, ops::RangeInclusive};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::map_res,
    sequence::{preceded, separated_pair},
    AsChar, IResult,
};

type TargetRange = RangeInclusive<i32>;
type Target = (TargetRange, TargetRange);
type Velocity = (i32, i32);
type Position = (i32, i32);

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, int) = map_res(
        take_while1(|c: char| c.is_dec_digit() || c == '-'),
        |s: &str| s.parse::<i32>(),
    )(input)?;

    Ok((input, int))
}

fn parse_target_range(input: &str) -> IResult<&str, TargetRange> {
    let (input, (from, to)) = preceded(
        alt((tag("x="), tag("y="))),
        separated_pair(parse_i32, tag(".."), parse_i32),
    )(input)?;

    Ok((input, from..=to))
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    let (input, targets) = preceded(
        tag("target area: "),
        separated_pair(parse_target_range, tag(", "), parse_target_range),
    )(input)?;

    Ok((input, targets))
}

fn step(position: &mut Position, velocity: &mut Velocity) {
    position.0 += velocity.0;
    position.1 += velocity.1;

    match velocity.0.cmp(&0) {
        Ordering::Greater => velocity.0 -= 1,
        Ordering::Equal => {}
        Ordering::Less => velocity.0 += 1,
    };

    velocity.1 -= 1;
}

fn in_target(position: &Position, target: &Target) -> bool {
    target.0.contains(&position.0) && target.1.contains(&position.1)
}

fn past_target(position: &Position, target: &Target) -> bool {
    &position.0 > target.0.end() || &position.1 < target.1.start()
}

fn positions(velocity: Velocity, target: &Target) -> Vec<Position> {
    let mut position = (0, 0);
    let mut velocity = velocity;

    let mut positions = Vec::new();

    while !past_target(&position, target) {
        step(&mut position, &mut velocity);

        positions.push(position);
    }

    positions
}

pub fn part1(input: &str) -> i32 {
    let target = parse_target(input).unwrap().1;

    let mut max_y = i32::MIN;

    for x in 0..=*target.0.end() {
        for y in -target.1.start().abs()..=target.1.start().abs() {
            let positions = positions((x, y), &target);
            if positions.iter().any(|p| in_target(p, &target)) {
                let y_height = positions.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

                if y_height > max_y {
                    max_y = y_height;
                }
            }
        }
    }

    max_y
}

pub fn part2(input: &str) -> u64 {
    let target = parse_target(input).unwrap().1;

    let mut velocities = 0;

    for x in 0..=*target.0.end() {
        for y in -target.1.start().abs()..=target.1.start().abs() {
            let positions = positions((x, y), &target);
            if positions.iter().any(|p| in_target(p, &target)) {
                velocities += 1;
            }
        }
    }

    velocities
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 45);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 112);
    }
}
