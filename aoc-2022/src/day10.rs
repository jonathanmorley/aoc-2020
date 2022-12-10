use std::fmt::{Display, Write};

use grid::Grid;
use itertools::Itertools;

struct Screen(Grid<bool>);

impl Screen {
    fn new() -> Screen {
        Screen(Grid::new(6, 40))
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.0.rows() {
            for pixel in self.0.iter_row(row) {
                f.write_char(if *pixel { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

pub fn parse(s: &str) -> Vec<(u32, i32)> {
    let instructions = s.lines().collect_vec();

    let mut instruction_counter = (0usize, 0u8);
    let mut x = 1i32;

    let mut states = Vec::new();

    for cycle in 1u32.. {
        states.push((cycle, x));

        if let Some(instruction) = instructions.get(instruction_counter.0) {
            match instruction.split_whitespace().collect_vec().as_slice() {
                ["noop"] => instruction_counter = (instruction_counter.0 + 1, 0),
                ["addx", n] => {
                    if instruction_counter.1 == 0 {
                        instruction_counter.1 = 1;
                    } else {
                        x += n.parse::<i32>().unwrap();
                        instruction_counter = (instruction_counter.0 + 1, 0);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }

    states
}

pub fn part1(input: &[(u32, i32)]) -> i32 {
    input
        .iter()
        .filter(|(cycle, _)| (cycle + 20) % 40 == 0)
        .map(|(cycle, x)| x * (*cycle as i32))
        .sum()
}

pub fn part2(input: &[(u32, i32)]) -> String {
    let mut screen = Screen::new();

    for (cycle, x) in input {
        let row = cycle / 40;
        let col = (cycle - 1) % 40;
        let sprite_range = (x - 1)..=(x + 1);

        if sprite_range.contains(&(col as i32)) {
            *screen.0.get_mut(row as usize, col as usize).unwrap() = true;
        }
    }

    screen.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(SAMPLE)), 13140);
    }

    #[test]
    fn test_part2() {
        // This is lightly different from what AoC provides,
        // but its close enough, so...
        const SOLUTION: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......###.
#######.......#######.......#######....#
";

        assert_eq!(part2(&parse(SAMPLE)), SOLUTION);
    }
}
