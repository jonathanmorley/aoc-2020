use std::collections::HashSet;

pub fn part1(s: &str) -> usize {
    s.as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, window)| {
            let mut uniq = HashSet::new();
            window.iter().all(move |x| uniq.insert(x))
        })
        .unwrap()
        .0
        + 4
}

pub fn part2(s: &str) -> usize {
    s.as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, window)| {
            let mut uniq = HashSet::new();
            window.iter().all(move |x| uniq.insert(x))
        })
        .unwrap()
        .0
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SAMPLE_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const SAMPLE_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const SAMPLE_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const SAMPLE_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 7);
        assert_eq!(part1(SAMPLE_2), 5);
        assert_eq!(part1(SAMPLE_3), 6);
        assert_eq!(part1(SAMPLE_4), 10);
        assert_eq!(part1(SAMPLE_5), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_1), 19);
        assert_eq!(part2(SAMPLE_2), 23);
        assert_eq!(part2(SAMPLE_3), 23);
        assert_eq!(part2(SAMPLE_4), 29);
        assert_eq!(part2(SAMPLE_5), 26);
    }
}
