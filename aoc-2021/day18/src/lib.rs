use std::iter::Sum;
use std::ops::{Add, RangeInclusive};
use std::str::FromStr;

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
struct Snailfish(String);

impl Snailfish {
    fn find_deep(&self) -> Option<usize> {
        let mut depth = 0;

        for (i, c) in self.0.char_indices() {
            if c == '[' {
                if depth == 4 {
                    return Some(i);
                }
                depth += 1;
            } else if c == ']' {
                depth -= 1;
            }
        }

        None
    }

    fn find_large(&self) -> Option<usize> {
        let mut iter = self.0.char_indices().peekable();
        while let Some((i, c)) = iter.next() {
            if c.is_numeric() && iter.peek().map(|(_, p)| p.is_numeric()) == Some(true) {
                //let end =
                return Some(i);
            }
        }

        None
    }

    fn regular_pair_range(&self, index: usize) -> RangeInclusive<usize> {
        let end = self.0[index..].chars().position(|c| c == ']').unwrap();
        index..=(index + end)
    }

    fn is_regular_pair(&self, index: usize) -> bool {
        if self.0.chars().nth(index) == Some('[') {
            let end = index + self.0[index..].chars().position(|c| c == ']').unwrap();
            self.0[index + 1..end]
                .chars()
                .all(|c| c == ',' || c.is_numeric())
        } else {
            false
        }
    }

    fn regular_range(&self, index: usize) -> RangeInclusive<usize> {
        let end = self.0[index..]
            .char_indices()
            .take_while(|(_, c)| c.is_numeric())
            .last()
            .unwrap()
            .0;

        index..=(index + end)
    }

    fn regular(&self, index: usize) -> u32 {
        let range = self.regular_range(index);
        u32::from_str(&self.0[range]).unwrap()
    }

    fn regular_pair(&self, index: usize) -> (u32, u32) {
        let first = self.regular(index + 1);
        let comma = self.0[index..].find(',').unwrap();
        let second = self.regular(index + comma + 1);
        (first, second)
    }

    fn explode(&mut self) -> bool {
        if let Some(deep) = self.find_deep() {
            let (left, right) = self.regular_pair(deep);

            let end = deep + self.0[deep..].find(']').unwrap() + 1;

            // Move from right to left so that we dont mess up subsequnt indices.

            // Add right
            if let Some(index) = &self.0[end..].chars().position(char::is_numeric) {
                let sum = right + self.regular(end + index);
                self.0
                    .replace_range(self.regular_range(end + index), &sum.to_string());
            }

            // Collapse
            self.0.replace_range(deep..end, "0");

            // Add left
            if let Some((index, _)) = &self.0[0..deep]
                .char_indices()
                .rfind(|(_, c)| c.is_numeric())
            {
                let index_left = self.0[..*index + 1]
                    .char_indices()
                    .rev()
                    .take_while(|(_, c)| c.is_numeric())
                    .last()
                    .unwrap()
                    .0;
                let sum = left + self.regular(index_left);
                self.0
                    .replace_range(self.regular_range(index_left), &sum.to_string());
            }

            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        if let Some(large) = self.find_large() {
            let range = self.regular_range(large);
            let value = self.regular(large);

            self.0
                .replace_range(range, &format!("[{},{}]", value / 2, (value + 1) / 2));
            true
        } else {
            false
        }
    }

    fn reduce(&mut self) {
        if self.explode() || self.split() {
            self.reduce();
        }
    }

    fn magnitude(self) -> u64 {
        let mut snailfish = self;

        while !snailfish.0.chars().all(char::is_numeric) {
            for i in 0..snailfish.0.len() {
                if snailfish.is_regular_pair(i) {
                    let (left, right) = snailfish.regular_pair(i);
                    let magnitude = 3 * left + 2 * right;
                    snailfish
                        .0
                        .replace_range(snailfish.regular_pair_range(i), &magnitude.to_string())
                }
            }
        }

        u64::from_str(&snailfish.0).unwrap()
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.0 == "" {
            return other;
        } else {
            let mut sum = Snailfish(format!("[{},{}]", self.0, other.0));
            sum.reduce();

            sum
        }
    }
}

impl Sum for Snailfish {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut acc = Snailfish(String::from(""));

        for snailfish in iter {
            acc = acc + snailfish;
        }

        acc
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| Snailfish(l.to_owned()))
        .sum::<Snailfish>()
        .magnitude()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| Snailfish(l.to_owned()))
        .tuple_combinations()
        .flat_map(|(a, b)| [a.clone() + b.clone(), b + a])
        .map(|sum| sum.magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::Snailfish;

    #[test]
    fn explode() {
        let mut sample = Snailfish(String::from("[[[[[9,8],1],2],3],4]"));
        sample.explode();
        assert_eq!(sample, Snailfish(String::from("[[[[0,9],2],3],4]")));

        let mut sample = Snailfish(String::from("[7,[6,[5,[4,[3,2]]]]]"));
        sample.explode();
        assert_eq!(sample, Snailfish(String::from("[7,[6,[5,[7,0]]]]")));

        let mut sample = Snailfish(String::from("[[6,[5,[4,[3,2]]]],1]"));
        sample.explode();
        assert_eq!(sample, Snailfish(String::from("[[6,[5,[7,0]]],3]")));

        let mut sample = Snailfish(String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));
        sample.explode();
        assert_eq!(
            sample,
            Snailfish(String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
        );

        let mut sample = Snailfish(String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        sample.explode();
        assert_eq!(
            sample,
            Snailfish(String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
        );

        let mut sample = Snailfish(String::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));
        sample.explode();
        assert_eq!(
            sample,
            Snailfish(String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"))
        );

        let mut sample = Snailfish(String::from(
            "[[[[4,0],[5,4]],[[7,7],[0,[6,7]]]],[10,[[11,0]],[[9,3],[8,8]]]]]",
        ));
        sample.explode();
        assert_eq!(
            sample,
            Snailfish(String::from(
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[17,[[11,0]],[[9,3],[8,8]]]]]"
            ))
        );

        let mut sample = Snailfish(String::from(
            "[[[[12,12],[6,14]],[[15,0],[17,[8,1]]]],[2,9]]",
        ));
        sample.explode();
        assert_eq!(
            sample,
            Snailfish(String::from("[[[[12,12],[6,14]],[[15,0],[25,0]]],[3,9]]"))
        );
    }

    #[test]
    fn split() {
        let mut sample = Snailfish(String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"));
        sample.split();
        assert_eq!(
            sample,
            Snailfish(String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"))
        );

        let mut sample = Snailfish(String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
        sample.split();
        assert_eq!(
            sample,
            Snailfish(String::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"))
        );
    }

    #[test]
    fn reduce() {
        let mut sample = Snailfish(String::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"));
        sample.reduce();
        assert_eq!(
            sample,
            Snailfish(String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"))
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Snailfish(String::from("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"))
                + Snailfish(String::from("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")),
            Snailfish(String::from(
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
            ))
        );
        assert_eq!(
            Snailfish(String::from(
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
            )) + Snailfish(String::from("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]")),
            Snailfish(String::from(
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
            ))
        );
        assert_eq!(
            Snailfish(String::from(
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
            )) + Snailfish(String::from(
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
            )),
            Snailfish(String::from(
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
            ))
        );
        assert_eq!(
            Snailfish(String::from(
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
            )) + Snailfish(String::from("[7,[5,[[3,8],[1,4]]]]")),
            Snailfish(String::from(
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
            ))
        );
        assert_eq!(
            Snailfish(String::from(
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
            )) + Snailfish(String::from("[[2,[2,2]],[8,[8,1]]]")),
            Snailfish(String::from(
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"
            ))
        );
        assert_eq!(
            Snailfish(String::from(
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"
            )) + Snailfish(String::from("[2,9]")),
            Snailfish(String::from(
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"
            ))
        );
    }

    #[test]
    fn sum() {
        let list = "[1,1]
[2,2]
[3,3]
[4,4]";
        assert_eq!(
            list.lines()
                .map(|l| Snailfish(l.to_owned()))
                .sum::<Snailfish>(),
            Snailfish(String::from("[[[[1,1],[2,2]],[3,3]],[4,4]]"))
        );

        let list = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";
        assert_eq!(
            list.lines()
                .map(|l| Snailfish(l.to_owned()))
                .sum::<Snailfish>(),
            Snailfish(String::from("[[[[3,0],[5,3]],[4,4]],[5,5]]"))
        );

        let list = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";
        assert_eq!(
            list.lines()
                .map(|l| Snailfish(l.to_owned()))
                .sum::<Snailfish>(),
            Snailfish(String::from("[[[[5,0],[7,4]],[5,5]],[6,6]]"))
        );

        let list = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        assert_eq!(
            list.lines()
                .map(|l| Snailfish(l.to_owned()))
                .sum::<Snailfish>(),
            Snailfish(String::from(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            ))
        );
    }

    #[test]
    fn magnitude() {
        //assert_eq!(Snailfish(String::from("[9,1]")).magnitude(), 29);
        assert_eq!(Snailfish(String::from("[[9,1],[1,9]]")).magnitude(), 129);
        assert_eq!(
            Snailfish(String::from("[[1,2],[[3,4],5]]")).magnitude(),
            143
        );
        assert_eq!(
            Snailfish(String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")).magnitude(),
            1384
        );
        assert_eq!(
            Snailfish(String::from("[[[[1,1],[2,2]],[3,3]],[4,4]]")).magnitude(),
            445
        );
        assert_eq!(
            Snailfish(String::from("[[[[3,0],[5,3]],[4,4]],[5,5]]")).magnitude(),
            791
        );
        assert_eq!(
            Snailfish(String::from("[[[[5,0],[7,4]],[5,5]],[6,6]]")).magnitude(),
            1137
        );
        assert_eq!(
            Snailfish(String::from(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            ))
            .magnitude(),
            3488
        );
    }

    const SAMPLE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 4140);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 3993);
    }
}
