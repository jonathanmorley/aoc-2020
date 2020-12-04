use aoc_runner_derive::aoc;

fn count_trees<'a>(lines: impl Iterator<Item = &'a str>, right: usize, down: usize) -> usize {
    lines
        .step_by(down)
        .enumerate()
        .map(|(index, line)| ((index * right) % line.len(), line))
        .map(|(index, line)| char::from(line.as_bytes()[index]))
        .filter(|&square| square == '#')
        .count()
}

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> usize {
    count_trees(input.lines(), 3, 1)
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> usize {
    [
        count_trees(input.lines(), 1, 1),
        count_trees(input.lines(), 3, 1),
        count_trees(input.lines(), 5, 1),
        count_trees(input.lines(), 7, 1),
        count_trees(input.lines(), 1, 2),
    ]
    .iter()
    .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_count_trees() {
        assert_eq!(count_trees(SAMPLE.lines(), 1, 1), 2);
        assert_eq!(count_trees(SAMPLE.lines(), 3, 1), 7);
        assert_eq!(count_trees(SAMPLE.lines(), 5, 1), 3);
        assert_eq!(count_trees(SAMPLE.lines(), 7, 1), 4);
        assert_eq!(count_trees(SAMPLE.lines(), 1, 2), 2);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE), 336);
    }
}
