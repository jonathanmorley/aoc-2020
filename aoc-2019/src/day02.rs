use aoc_runner_derive::aoc;

use anyhow::{Result, bail};
use crate::intcode::Stack;

#[aoc(day2, part1)]
pub fn part_1(input: &str) -> Result<usize> {
    let mut stack: Stack = input.parse()?;

    stack.input(12, 2);

    stack.run()?;
    stack.output()
}

#[aoc(day2, part2)]
pub fn part_2(input: &str) -> Result<usize> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut stack: Stack = input.parse()?;
            stack.input(noun, verb);
            stack.run()?;

            if stack.output()? == 19690720 {
                return Ok(100 * noun + verb)
            }
        }
    }

    bail!("No valid inputs found");
}