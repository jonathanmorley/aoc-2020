use std::{
    collections::HashSet,
    fmt::{self, Write},
};

use anyhow::Result;

#[derive(Clone, Debug)]
struct OctopusGrid(Vec<Vec<u16>>);

type Point = (usize, usize);

impl fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for line in &self.0 {
            for energy in line {
                write!(f, "{:<3}", energy)?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl OctopusGrid {
    fn step(&mut self) -> usize {
        self.increment();
        self.flashes();
        self.normalize();
        self.count_flashes()
    }

    fn increment(&mut self) {
        for line in self.0.iter_mut() {
            for energy in line {
                *energy += 1;
            }
        }
    }

    fn flashes(&mut self) {
        let mut flashed = HashSet::new();

        loop {
            let mut flashers = Vec::new();

            for x in 0..self.0.len() {
                for y in 0..self.0[x].len() {
                    if self.0[x][y] >= 10 && !flashed.contains(&(x, y)) {
                        flashers.push((x, y));
                    }
                }
            }

            if flashers.is_empty() {
                break;
            } else {
                for flasher in flashers {
                    self.flash(flasher);
                    flashed.insert(flasher);
                }
            }
        }
    }

    fn flash(&mut self, point: Point) {
        // increment self so that its not always 9
        // and triggers flashing
        self.0[point.0][point.1] += 1;

        for (x, y) in self.neighbours(point) {
            self.0[x][y] += 1;
        }
    }

    fn neighbours(&self, (x, y): Point) -> Vec<Point> {
        vec![
            if x > 0 { Some((x - 1, y)) } else { None },
            if y > 0 { Some((x, y - 1)) } else { None },
            if x < self.0.len() - 1 {
                Some((x + 1, y))
            } else {
                None
            },
            if y < self.0[x].len() - 1 {
                Some((x, y + 1))
            } else {
                None
            },
            if x > 0 && y > 0 {
                Some((x - 1, y - 1))
            } else {
                None
            },
            if x > 0 && y < self.0[x].len() - 1 {
                Some((x - 1, y + 1))
            } else {
                None
            },
            if x < self.0.len() - 1 && y > 0 {
                Some((x + 1, y - 1))
            } else {
                None
            },
            if x < self.0.len() - 1 && y < self.0[x].len() - 1 {
                Some((x + 1, y + 1))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn normalize(&mut self) {
        for line in self.0.iter_mut() {
            for energy in line {
                if *energy > 9 {
                    *energy = 0;
                }
            }
        }
    }

    fn count_flashes(&self) -> usize {
        self.0
            .iter()
            .flat_map(|line| line.iter().filter(|energy| energy == &&0))
            .count()
    }
}

fn generator(input: &str) -> OctopusGrid {
    OctopusGrid(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect()
            })
            .collect(),
    )
}

pub fn part1(input: &str) -> usize {
    let input = generator(input);
    let mut octopodes = input;

    (0..100).map(|_| octopodes.step()).sum()
}

pub fn part2(input: &str) -> usize {
    let input = generator(input);
    let mut octopodes = input;

    (1..)
        .map(|step| (step, octopodes.step()))
        .find(|(_, flashes)| *flashes == 100)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 1656);
    }

    #[test]
    fn sample2() {
        assert_eq!(super::part2(SAMPLE), 195);
    }
}
