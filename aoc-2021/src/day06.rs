use anyhow::Result;

fn generator(input: &str) -> [u64; 9] {
    input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .fold([0; 9], |mut acc, fish: usize| {
            acc[fish] += 1;
            acc
        })
}

fn lanternfish(initial: Vec<u64>, day: u32) -> u64 {
    let mut school = initial;

    for _ in 0..day {
        school.rotate_left(1);
        school[6] += school[8];
    }

    school.into_iter().sum()
}

pub fn part1(input: &str) -> u64 {
    let input = generator(input);
    lanternfish(Vec::from_iter(input.to_owned()), 80)
}

pub fn part2(input: &str) -> u64 {
    let input = generator(input);
    lanternfish(Vec::from_iter(input.to_owned()), 256)
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 5934);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 26984457539);
    }
}
