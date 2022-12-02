use thiserror::Error;

fn generator(input: &str) -> Vec<String> {
    input.lines().map(ToOwned::to_owned).collect()
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Incomplete expression. Complete by adding {expected}")]
    Incomplete { expected: String },
    #[error("Expected {expected}, but found {found} instead")]
    Corrupted { expected: char, found: char },
}

impl ParseError {
    fn score(&self) -> u64 {
        match self {
            ParseError::Corrupted { found, .. } => match found {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            },
            ParseError::Incomplete { ref expected } => expected.chars().fold(0, |score, c| {
                score * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            }),
        }
    }
}

fn terminator(c: &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn parse_brackets(s: &str) -> Result<(), ParseError> {
    let mut stack = Vec::new();

    for c in s.chars() {
        match (stack.last(), c) {
            (_, '(' | '[' | '{' | '<') => {
                stack.push(c);
            }
            (Some('('), ')') => {
                stack.pop();
            }
            (Some('['), ']') => {
                stack.pop();
            }
            (Some('{'), '}') => {
                stack.pop();
            }
            (Some('<'), '>') => {
                stack.pop();
            }
            (Some(pop), e) => {
                return Err(ParseError::Corrupted {
                    expected: terminator(pop),
                    found: e,
                });
            }
            _ => unreachable!(),
        }
    }

    if stack.is_empty() {
        Ok(())
    } else {
        Err(ParseError::Incomplete {
            expected: stack.iter().map(terminator).rev().collect(),
        })
    }
}

pub fn part1(input: &str) -> u64 {
    let input = generator(input);

    input
        .iter()
        .map(|line| parse_brackets(line))
        .filter_map(|result| {
            if let Err(e @ ParseError::Corrupted { .. }) = result {
                Some(e.score())
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let input = generator(input);

    let mut scores: Vec<_> = input
        .iter()
        .map(|line| parse_brackets(line))
        .filter_map(|result| {
            if let Err(e @ ParseError::Incomplete { .. }) = result {
                Some(e.score())
            } else {
                None
            }
        })
        .collect();

    let n = scores.len() / 2;
    *scores.select_nth_unstable(n).1
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 26397);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 288957);
    }
}
