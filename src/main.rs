use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
enum Args {
  /// Solve a day
  Solve {
    #[clap(short, long, default_value_t = 2021)]
    year: u16,
    #[clap(short, long)]
    day: u8
  }
}

macro_rules! solve {
  ($year:literal, $day:literal, $c:path) => {
    {
      use $c as c;
      use std::time::Instant;

      const INPUT: &str = include_str!(concat!("../input/", $year, "/day", $day, ".txt"));
      println!("AOC {}", $year);
      
      {
        let start_time = Instant::now();
        let solution = c::part1(INPUT);
        let final_time = Instant::now();

        println!("Day {} - Part 1 : {}", $day, solution);
        println!("runner: {:?}", final_time - start_time);
      }
      
      println!();

      {
        let start_time = Instant::now();
        let solution = c::part2(INPUT);
        let final_time = Instant::now();

        println!("Day {} - Part 2 : {}", $day, solution);
        println!("runner: {:?}", final_time - start_time);
      }
    }
  };
}

fn main() {
  match Args::parse() {
    Args::Solve { year: 2021, day: 01 } => solve!(2021, 01, aoc_2021_day01),
    Args::Solve { year: 2021, day: 02 } => solve!(2021, 02, aoc_2021_day02),
    Args::Solve { year: 2021, day: 03 } => solve!(2021, 03, aoc_2021_day03),
    Args::Solve { year: 2021, day: 04 } => solve!(2021, 04, aoc_2021_day04),
    Args::Solve { year: 2021, day: 05 } => solve!(2021, 05, aoc_2021_day05),
    Args::Solve { year: 2021, day: 06 } => solve!(2021, 06, aoc_2021_day06),
    Args::Solve { year: 2021, day: 07 } => solve!(2021, 07, aoc_2021_day07),
    Args::Solve { year: 2021, day: 08 } => solve!(2021, 08, aoc_2021_day08),
    Args::Solve { year: 2021, day: 09 } => solve!(2021, 09, aoc_2021_day09),
    Args::Solve { year: 2021, day: 10 } => solve!(2021, 10, aoc_2021_day10),
    Args::Solve { year: 2021, day: 11 } => solve!(2021, 11, aoc_2021_day11),
    Args::Solve { year: 2021, day: 12 } => solve!(2021, 12, aoc_2021_day12),
    Args::Solve { year: 2021, day: 13 } => solve!(2021, 13, aoc_2021_day13),
    Args::Solve { year: 2021, day: 14 } => solve!(2021, 14, aoc_2021_day14),
    Args::Solve { year: 2021, day: 15 } => solve!(2021, 15, aoc_2021_day15),
    Args::Solve { year: 2021, day: 16 } => solve!(2021, 16, aoc_2021_day16),
    Args::Solve { year: 2021, day: 17 } => solve!(2021, 17, aoc_2021_day17),
    // Args::Solve { year: 2021, day: 18 } => solve!(2021, 18, aoc_2021_day18),
    // Args::Solve { year: 2021, day: 19 } => solve!(2021, 19, aoc_2021_day19),
    // Args::Solve { year: 2021, day: 20 } => solve!(2021, 20, aoc_2021_day20),
    // Args::Solve { year: 2021, day: 21 } => solve!(2021, 21, aoc_2021_day21),
    // Args::Solve { year: 2021, day: 22 } => solve!(2021, 22, aoc_2021_day22),
    // Args::Solve { year: 2021, day: 23 } => solve!(2021, 23, aoc_2021_day23),
    // Args::Solve { year: 2021, day: 24 } => solve!(2021, 24, aoc_2021_day24),
    // Args::Solve { year: 2021, day: 25 } => solve!(2021, 25, aoc_2021_day25),
    _ => unreachable!()
  }
}

