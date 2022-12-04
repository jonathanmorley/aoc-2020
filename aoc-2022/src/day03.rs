pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    input
        .into_iter()
        .map(|line| (&line[..line.len() / 2], &line[line.len() / 2..]))
        .filter_map(|rucksack| {
            for c in rucksack.0.chars() {
                if rucksack.1.chars().find(|other| other.eq(&c)).is_some() {
                    if c.is_uppercase() {
                        return Some(c as u32 - 38);
                    } else {
                        return Some(c as u32 - 96);
                    }
                }
            }

            None
        })
        .sum::<u32>()
}

pub fn part2(input: &[&str]) -> u32 {
    input
        .chunks_exact(3)
        .filter_map(|group| {
            for c in group[0].chars() {
                if group[1].chars().find(|other| other.eq(&c)).is_some() {
                    if group[2].chars().find(|other| other.eq(&c)).is_some() {
                        if c.is_uppercase() {
                            return Some(c as u32 - 38);
                        } else {
                            return Some(c as u32 - 96);
                        }
                    }
                }
            }

            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(SAMPLE)), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(SAMPLE)), 70);
    }
}
