use std::collections::BTreeMap;

pub type Crates = BTreeMap<usize, Vec<char>>;

#[derive(Debug)]
pub struct Instruction {
    n: u8,
    from: usize,
    to: usize,
}

pub fn parse(input: &str) -> (Crates, Vec<Instruction>) {
    let (crate_lines, instructions) = input.split_once("\n\n").unwrap();

    let mut crates: Crates = BTreeMap::new();
    for crate_line in crate_lines.lines().rev().skip(1) {
        for (idx, c) in crate_line
            .char_indices()
            .filter(|(_, c)| c.is_ascii_alphabetic())
        {
            let idx = ((idx - 1) / 4) + 1;
            if let Some(stack) = crates.get_mut(&idx) {
                stack.push(c);
            } else {
                crates.insert(idx, vec![c]);
            }
        }
    }

    let instructions = instructions
        .lines()
        .map(|i| {
            let mut parts = i.split_whitespace();

            Instruction {
                n: parts.nth(1).unwrap().parse().unwrap(),
                from: parts.nth(1).unwrap().parse().unwrap(),
                to: parts.nth(1).unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    (crates, instructions)
}

pub fn part1((crates, instructions): &(Crates, Vec<Instruction>)) -> String {
    let mut crates = crates.clone();

    for Instruction { n, from, to } in instructions {
        for _ in 0..*n {
            let _crate = crates.get_mut(from).unwrap().pop().unwrap();
            crates.get_mut(to).unwrap().push(_crate);
        }
    }

    crates
        .into_values()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

pub fn part2((crates, instructions): &(Crates, Vec<Instruction>)) -> String {
    let mut crates = crates.clone();

    for Instruction { n, from, to } in instructions {
        let mut crate_mover_9001_stack = Vec::new();

        for _ in 0..*n {
            crate_mover_9001_stack.push(crates.get_mut(from).unwrap().pop().unwrap());
        }

        for _ in 0..*n {
            crates
                .get_mut(to)
                .unwrap()
                .push(crate_mover_9001_stack.pop().unwrap());
        }
    }

    crates
        .into_values()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(SAMPLE)), String::from("CMZ"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(SAMPLE)), String::from("MCD"));
    }
}
