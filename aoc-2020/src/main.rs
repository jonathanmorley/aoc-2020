mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
//mod day07;

aoc_main::main! {
  year 2020;
  day01 :parse?     => part_1?, part_2?;
  day02             => part_1, part_2;
  day03             => part_1, part_2;
  day04             => part_1, part_2;
  day05 :parse?     => part_1?, part_2?;
  day06 :parse?     => part_1, part_2;
  //day07             => part1, part2;
}