use nom::{
    IResult,
    bytes::complete::tag,
    combinator::map_res, Finish, character::complete::digit1, multi::separated_list1};
use nom::combinator::recognize;
use nom::branch::alt;
use nom::character::complete::char;
use nom::sequence::separated_pair;

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32
}

impl CubeSet {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, counts): (_, Vec<(u32, _)>) = separated_list1(tag(", "),
                separated_pair(map_res(recognize(digit1), str::parse), char(' '), alt((tag("red"), tag("green"), tag("blue"))))
        )(input)?;

        Ok((input, CubeSet {
            red: counts.iter().find_map(|(count, colour)| if *colour == "red" { Some(*count) } else { None }).unwrap_or(0),
            green: counts.iter().find_map(|(count, colour)| if *colour == "green" { Some(*count) } else { None }).unwrap_or(0),
            blue: counts.iter().find_map(|(count, colour)| if *colour == "blue" { Some(*count) } else { None }).unwrap_or(0),
        }))
    }

    fn power(&self) -> u64 {
        self.red as u64 * self.green as u64 * self.blue as u64
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    selections: Vec<CubeSet>
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = map_res(recognize(digit1), str::parse)(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, selections) = separated_list1(tag("; "), CubeSet::parse)(input)?;
        
        Ok((input, Game { id, selections }))
    }

    fn minimal(&self) -> CubeSet {
        CubeSet {
            red: self.selections.iter().map(|s| s.red).max().unwrap(),
            green: self.selections.iter().map(|s| s.green).max().unwrap(),
            blue: self.selections.iter().map(|s| s.blue).max().unwrap(),
        }
    }
}

fn generator(input: &str) -> Result<Vec<Game>, nom::error::Error<&str>> {
    input.lines()
        .map(|l| Game::parse(l).finish().map(|(_, game)| game))
        .collect()
}

pub fn part1(input: &str) -> u32 {
   generator(input).unwrap()
        .into_iter()
        .filter(|game| game.selections.iter().all(|selection| selection.red <= 12 && selection.green <= 13 && selection.blue <= 14))
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    generator(input).unwrap()
         .into_iter()
         .map(|game| game.minimal().power())
         .sum()
 }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_1), 2286);
    }
}
