use std::collections::HashMap;

use petgraph::algo::astar::astar;
use petgraph::graphmap::DiGraphMap;

pub fn generator(input: &str) -> HashMap<(i32, i32), u64> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap() as u64))
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let nodes = generator(input);

    let mut graph = DiGraphMap::new();

    for location in nodes.keys() {
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let neighbour = (location.0 + dx, location.1 + dy);

            if let Some(weight) = nodes.get(&neighbour) {
                graph.add_edge(*location, neighbour, *weight);
            }
        }
    }

    let max = graph.nodes().max().unwrap();

    astar(
        &graph,
        (0, 0),
        |finish| finish == max,
        |(_, _, weight)| *weight,
        |(x, y)| ((max.0 - x) + (max.1 - y)).unsigned_abs() as u64,
    )
    .unwrap()
    .0
}

pub fn part2(input: &str) -> u64 {
    let mut nodes = generator(input);

    let min = nodes.keys().min().unwrap().to_owned();
    let max = nodes.keys().max().unwrap().to_owned();
    let (span_x, span_y) = (max.0 - min.0 + 1, max.1 - min.1 + 1);

    // Stretch horizontally
    for ((x, y), weight) in nodes.clone() {
        for i in 1..5 {
            nodes.insert(
                (x + (span_x * i), y),
                ((((weight as i32 + i) - 1) % 9) + 1) as u64,
            );
        }
    }

    // Stretch vertically
    for ((x, y), weight) in nodes.clone() {
        for i in 1..5 {
            nodes.insert(
                (x, y + (span_y * i)),
                ((((weight as i32 + i) - 1) % 9) + 1) as u64,
            );
        }
    }

    let mut graph = DiGraphMap::new();

    for location in nodes.keys() {
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let neighbour = (location.0 + dx, location.1 + dy);

            if let Some(weight) = nodes.get(&neighbour) {
                graph.add_edge(*location, neighbour, *weight);
            }
        }
    }

    let max = graph.nodes().max().unwrap();

    astar(
        &graph,
        (0, 0),
        |finish| finish == max,
        |(_, _, weight)| *weight,
        |(x, y)| ((max.0 - x) + (max.1 - y)).unsigned_abs() as u64,
    )
    .unwrap()
    .0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn sample1() {
        assert_eq!(part1(SAMPLE), 40);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(SAMPLE), 315);
    }
}
