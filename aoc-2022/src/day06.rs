use std::collections::HashSet;

fn is_unique_hashset(slice: &[u8]) -> bool {
    let mut uniq = HashSet::new();
    slice.iter().all(move |x| uniq.insert(x))
}

fn is_unique_loops(slice: &[u8]) -> bool {
    for i in 0..slice.len() {
        for j in i + 1..slice.len() {
            if slice[i] == slice[j] {
                return false;
            }
        }
    }
    true
}

pub fn part1(s: &str) -> usize {
    s.as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, window)| is_unique_hashset(window))
        .unwrap()
        .0
        + 4
}

pub fn part2_hashset(s: &str) -> usize {
    s.as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, window)| is_unique_hashset(window))
        .unwrap()
        .0
        + 14
}

pub fn part2_loops(s: &str) -> usize {
    s.as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, window)| is_unique_loops(window))
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
    fn test_part2_hashset() {
        assert_eq!(part2_hashset(SAMPLE_1), 19);
        assert_eq!(part2_hashset(SAMPLE_2), 23);
        assert_eq!(part2_hashset(SAMPLE_3), 23);
        assert_eq!(part2_hashset(SAMPLE_4), 29);
        assert_eq!(part2_hashset(SAMPLE_5), 26);
    }

    #[test]
    fn test_part2_loops() {
        assert_eq!(part2_loops(SAMPLE_1), 19);
        assert_eq!(part2_loops(SAMPLE_2), 23);
        assert_eq!(part2_loops(SAMPLE_3), 23);
        assert_eq!(part2_loops(SAMPLE_4), 29);
        assert_eq!(part2_loops(SAMPLE_5), 26);
    }
}
