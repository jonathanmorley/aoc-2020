use core::str::FromStr;
use nom::character::complete::char;
use nom::character::complete::space1;
use nom::combinator::recognize;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    Finish, IResult,
};
use std::collections::HashSet;

#[derive(Debug)]
struct Card<'a> {
    id: u32,
    winners: HashSet<&'a str>,
    numbers: HashSet<&'a str>,
}

fn parse_number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(digit1), str::parse)(input)
}

impl<'a> Card<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, _) = tuple((tag("Card"), space1))(input)?;
        let (input, id) = parse_number(input)?;
        let (input, _) = tuple((char(':'), space1))(input)?;
        let (input, (winners, numbers)) = separated_pair(
            separated_list1(space1, digit1),
            tuple((space1, char('|'), space1)),
            separated_list1(space1, digit1),
        )(input)?;

        Ok((
            input,
            Card {
                id,
                winners: HashSet::from_iter(winners),
                numbers: HashSet::from_iter(numbers),
            },
        ))
    }

    fn matches(&self) -> u32 {
        self
            .numbers
            .iter()
            .filter(|number| self.winners.contains(*number))
            .count() as u32
    }

    fn score(&self) -> u32 {
        if self.matches() == 0 {
            0
        } else {
            2_u32.pow(self.matches() - 1)
        }
    }
}

fn generator(input: &str) -> Result<Vec<Card>, nom::error::Error<&str>> {
    input
        .lines()
        .map(|l| Card::parse(l).finish().map(|(_, card)| card))
        .collect()
}

pub fn part1(input: &str) -> u32 {
    generator(input).unwrap().into_iter().map(|c| c.score()).sum()
}

pub fn part2(input: &str) -> u32 {
    let cards = generator(input).unwrap();

    let card_indices: Vec<_> = cards.iter().map(|card| card.id..(card.id+card.matches())).collect();

    dbg!(card_indices);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_1), 30);
    }
}
