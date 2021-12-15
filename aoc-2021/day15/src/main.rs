use aoc_2021_day15::{generator, part1, part2};

const INPUT: &str = include_str!("../../../input/2021/day15.txt");

fn main() {
  println!("AOC 2021 Day 15");
  println!("Part 1: {}", part1(generator(INPUT)));
  println!("Part 2: {}", part2(generator(INPUT)));
}