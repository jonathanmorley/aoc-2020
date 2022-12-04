mod day01;
mod day02;
//mod day03;
mod intcode;

aoc_main::main! {
  year 2019;
  day01 :parse?     => part_1, part_2;
  day02             => part_1?, part_2?;
  //day03 :parse?     => part_1, part_2;
}
