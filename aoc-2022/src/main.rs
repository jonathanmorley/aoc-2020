mod day01;
mod day02;
mod day03;
mod day04;

aoc_main::main! {
  year 2022;
  day01             => part1, part2;
  day02             => part1, part2;
  day03 :parse      => part1, part2;
  day04 :parse      => part1, part2;
}
