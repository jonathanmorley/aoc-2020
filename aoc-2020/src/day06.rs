use std::{collections::HashSet, hash::Hash, hash::Hasher, str::FromStr};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
pub struct Answer(HashSet<char>);

impl FromStr for Answer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Answer(s.chars().filter(|&c| c != '\n').collect()))
    }
}

impl Hash for Answer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for value in &self.0 {
            value.hash(state);
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AnswerSet(Vec<Answer>);

impl FromStr for AnswerSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AnswerSet(
            s.lines().map(Answer::from_str).collect::<Result<_>>()?,
        ))
    }
}

impl AnswerSet {
    fn any(&self) -> HashSet<&char> {
        self.0.iter().flat_map(|answer| answer.0.iter()).collect()
    }

    fn all(&self) -> Option<HashSet<&char>> {
        self.0.iter().fold(None, |acc: Option<HashSet<&char>>, hs| {
            let hs = hs.0.iter().collect();
            acc.map(|a| a.intersection(&hs).copied().collect())
                .or(Some(hs))
        })
    }
}

pub fn parse(input: &str) -> Result<Vec<AnswerSet>> {
    input.split("\n\n").map(AnswerSet::from_str).collect()
}

pub fn part_1(input: &[AnswerSet]) -> usize {
    input.iter().map(|set| set.any().len()).sum()
}

pub fn part_2(input: &[AnswerSet]) -> usize {
    input
        .iter()
        .map(|set| set.all().map(|hs| hs.len()).unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(
            parse(SAMPLE)?,
            vec![
                AnswerSet(vec![Answer(vec!['a', 'c', 'b'].into_iter().collect())]),
                AnswerSet(vec![
                    Answer(vec!['a'].into_iter().collect()),
                    Answer(vec!['b'].into_iter().collect()),
                    Answer(vec!['c'].into_iter().collect())
                ]),
                AnswerSet(vec![
                    Answer(vec!['a', 'b'].into_iter().collect()),
                    Answer(vec!['a', 'c'].into_iter().collect()),
                ]),
                AnswerSet(vec![
                    Answer(vec!['a'].into_iter().collect()),
                    Answer(vec!['a'].into_iter().collect()),
                    Answer(vec!['a'].into_iter().collect()),
                    Answer(vec!['a'].into_iter().collect())
                ]),
                AnswerSet(vec![Answer(vec!['b'].into_iter().collect())]),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_any() -> Result<()> {
        let parsed = parse(SAMPLE)?;
        assert_eq!(
            parsed.iter().map(|set| set.any()).collect::<Vec<_>>(),
            vec![
                vec!['a', 'b', 'c'].iter().collect(),
                vec!['a', 'b', 'c'].iter().collect(),
                vec!['a', 'b', 'c'].iter().collect(),
                vec!['a'].iter().collect(),
                vec!['b'].iter().collect()
            ]
        );
        Ok(())
    }

    #[test]
    fn test_all() -> Result<()> {
        let parsed = parse(SAMPLE)?;
        assert_eq!(
            parsed.iter().map(|set| set.all()).collect::<Vec<_>>(),
            vec![
                Some(vec!['a', 'b', 'c'].iter().collect()),
                Some(vec![].iter().collect()),
                Some(vec!['a'].iter().collect()),
                Some(vec!['a'].iter().collect()),
                Some(vec!['b'].iter().collect())
            ]
        );
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let parsed = parse(SAMPLE)?;
        assert_eq!(part_1(&parsed), 11);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let parsed = parse(SAMPLE)?;
        assert_eq!(part_2(&parsed), 6);
        Ok(())
    }
}
