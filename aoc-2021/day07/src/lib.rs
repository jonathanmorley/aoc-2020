use anyhow::Result;

fn generator(input: &str) -> Result<Vec<u32>> {
    input
        .split(",")
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}

pub fn part1(input: &str) -> u32 {
    let input = generator(input).unwrap();

    let (min, max) = (*input.iter().min().unwrap(), *input.iter().max().unwrap());

    (min..=max)
        .map(|align| {
            input
                .iter()
                .map(|crab| (*crab as i64 - align as i64).abs() as u32)
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let input = generator(input).unwrap();

    let (min, max) = (*input.iter().min().unwrap(), *input.iter().max().unwrap());

    (min..=max)
        .map(|align| {
            input
                .iter()
                .map(|crab| (*crab as i64 - align as i64).abs() as u32)
                .map(|distance| (distance * (distance + 1)) / 2)
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 37);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 168);
    }
}
