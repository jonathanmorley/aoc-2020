use itertools::Itertools;
use petgraph::Graph;

trait Grid {
    fn get(&self, at: (usize, usize)) -> Option<&u8>;
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn delta(&self, src: (usize, usize), dst: (usize, usize)) -> Option<u8>;
}

impl Grid for &str {
    fn get(&self, (row, col): (usize, usize)) -> Option<&u8> {
        self.lines().nth(row).and_then(|line| line.as_bytes().get(col))
    }

    fn rows(&self) -> usize {
        self.lines().count()
    }

    fn cols(&self) -> usize {
        self.find('\n').unwrap()
    }

    fn delta(&self, src: (usize, usize), dst: (usize, usize)) -> Option<u8> {
        self.get(dst).and_then(|dst| self.get(src).map(|src| dst - src))
    }
}

#[derive(Debug)]
pub struct Map {
    heightmap: Graph<(usize, usize), u32>,
    start: (usize, usize),
    end: (usize, usize)
}

pub fn parse(input: &str) -> Map {
    // let mut start = None;
    // let mut end = None;

    let mut heightmap = Graph::new();

    for row in 0..input.rows() {
        for col in 0..input.cols() {
            heightmap.add_node((row, col));

            

            dbg!((row, col));
        }
    }

    todo!()

    // for line in input.lines() {
    //     heightmap.push_row(line.chars().collect_vec());
    // }

    // for (row, col) in heightmap.points() {
    //     let height = heightmap.get_mut(row, col).unwrap();
    //     if *height == 'S' {
    //         *height = 'a';
    //         start = Some((row, col));
    //     } else if *height == 'E' {
    //         *height = 'z';
    //         end = Some((row, col));
    //     }
    // }

    // Map {
    //     heightmap,
    //     start: start.unwrap(),
    //     end: end.unwrap()
    // }
}

pub fn part1(input: &Map) -> u128 {
    dbg!(input);
    todo!()
}

pub fn part2(input: &Map) -> u128 {

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(SAMPLE)), 31);
    }
}
