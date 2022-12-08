use grid::Grid;
use itertools::Itertools;

type Forest = Grid<u32>;

trait GridExt {
    fn points(&self) -> Vec<(usize, usize)>;
    fn views(&self, row: usize, col: usize) -> [Vec<&u32>; 4];
    fn restricted_views(&self, row: usize, col: usize) -> [Vec<&u32>; 4];
    fn is_externally_visible(&self, row: usize, col: usize) -> bool;
    fn score(&self, row: usize, col: usize) -> usize;
}

impl GridExt for Forest {
    fn points(&self) -> Vec<(usize, usize)> {
        (0..self.rows())
            .flat_map(|row| (0..self.cols()).map(move |col| (row, col)))
            .collect()
    }

    fn views(&self, row: usize, col: usize) -> [Vec<&u32>; 4] {
        [
            self.iter_row(row).skip(col + 1).collect(),
            self.iter_col(col).skip(row + 1).collect(),
            self.iter_row(row).rev().skip(self.cols() - col).collect(),
            self.iter_col(col).rev().skip(self.rows() - row).collect(),
        ]
    }

    fn restricted_views(&self, row: usize, col: usize) -> [Vec<&u32>; 4] {
        let height = self.get(row, col).unwrap();

        self.views(row, col).map(|view| {
            let mut view = view.into_iter();
            let mut restricted_view = view
                .take_while_ref(|elem| *elem < height)
                .collect::<Vec<_>>();

            if let Some(tree) = view.next() {
                restricted_view.push(tree);
            }

            restricted_view
        })
    }

    fn score(&self, row: usize, col: usize) -> usize {
        self.restricted_views(row, col)
            .map(|view| view.len())
            .iter()
            .product()
    }

    fn is_externally_visible(&self, row: usize, col: usize) -> bool {
        let height = self.get(row, col).unwrap();

        self.views(row, col)
            .iter()
            .any(|view| view.iter().all(|tree| *tree < height))
    }
}

pub fn parse(s: &str) -> Forest {
    let mut forest = Forest::new(0, 0);

    for line in s.lines() {
        forest.push_row(
            line.chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()
                .unwrap(),
        );
    }

    forest
}

pub fn part1(forest: &Forest) -> usize {
    forest
        .points()
        .iter()
        .filter(|(row, col)| forest.is_externally_visible(*row, *col))
        .count()
}

pub fn part2(forest: &Forest) -> usize {
    forest
        .points()
        .iter()
        .map(|(row, col)| forest.score(*row, *col))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(SAMPLE)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(SAMPLE)), 8);
    }
}
