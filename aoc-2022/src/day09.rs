use itertools::Itertools;

type Location = (i32, i32);

pub fn parse(s: &str) -> Vec<&str> {
    s.lines().collect()
}

fn update(head: &Location, tail: Location) -> Location {
    match (head.0 - tail.0, head.1 - tail.1) {
        (0, 0) => tail,                                // overlap
        (-1, 0) | (1, 0) => tail,                      // touching horizontally
        (0, -1) | (0, 1) => tail,                      // touching vertically
        (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => tail, // touching diagonally
        (2, 0) => (tail.0 + 1, tail.1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (0, 2) => (tail.0, tail.1 + 1),
        (0, -2) => (tail.0, tail.1 - 1),
        (2, 1) | (1, 2) => (tail.0 + 1, tail.1 + 1),
        (2, -1) | (1, -2) => (tail.0 + 1, tail.1 - 1),
        (-2, -1) | (-1, -2) => (tail.0 - 1, tail.1 - 1),
        (-2, 1) | (-1, 2) => (tail.0 - 1, tail.1 + 1),
        // new diagonal motions
        (2, 2) => (tail.0 + 1, tail.1 + 1),
        (2, -2) => (tail.0 + 1, tail.1 - 1),
        (-2, -2) => (tail.0 - 1, tail.1 - 1),
        (-2, 2) => (tail.0 - 1, tail.1 + 1),
        _ => unreachable!(),
    }
}

pub fn part1(input: &[&str]) -> usize {
    let mut tail_history: Vec<Location> = Vec::new();
    let mut head = Location::default();
    let mut tail = Location::default();

    for line in input {
        if let Some((direction, n)) = line.split_once(' ') {
            for _ in 0..n.parse().unwrap() {
                match direction {
                    "L" => head.0 -= 1,
                    "R" => head.0 += 1,
                    "D" => head.1 -= 1,
                    "U" => head.1 += 1,
                    _ => unreachable!(),
                }

                tail = update(&head, tail);
                tail_history.push(tail);
            }
        }
    }

    tail_history.into_iter().unique().count()
}

pub fn part2(input: &[&str]) -> usize {
    const KNOTS: usize = 10;

    let mut tail_history: Vec<Location> = Vec::new();
    let mut knots = [Location::default(); KNOTS];

    for line in input {
        if let Some((direction, n)) = line.split_once(' ') {
            for _ in 0..n.parse().unwrap() {
                // move head
                match direction {
                    "L" => knots.first_mut().unwrap().0 -= 1,
                    "R" => knots.first_mut().unwrap().0 += 1,
                    "D" => knots.first_mut().unwrap().1 -= 1,
                    "U" => knots.first_mut().unwrap().1 += 1,
                    _ => unreachable!(),
                }

                // move the rest of the rope
                for i in 1..knots.len() {
                    knots[i] = update(&knots[i - 1], knots[i]);
                }

                // track the motion of the last knot
                tail_history.push(knots.last().unwrap().to_owned());
            }
        }
    }

    tail_history.into_iter().unique().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const SAMPLE_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(SAMPLE_1)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(SAMPLE_1)), 1);
        assert_eq!(part2(&parse(SAMPLE_2)), 36);
    }
}
