use aoc_2021_day01::{generator, part1, part2};

const INPUT: &str = include_str!("../../../input/2021/day1.txt");

fn main() {
  println!("AOC 2021");
  println!("Part 1: {}", part1(&generator(INPUT).unwrap()));
  println!("Part 2: {}", part2(&generator(INPUT).unwrap()));
}