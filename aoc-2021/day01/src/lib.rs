use std::iter;
use std::num::ParseIntError;

use itertools::izip;

fn generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

pub fn part1(input: &str) -> usize {
    let input = generator(input).unwrap();

    let offset = iter::once(None).chain(input.clone().into_iter().map(Some));

    // Creates an iterator of (Option<u8>, u8),
    // representing the previous value, and the current value.
    let cmp = offset.zip(input);

    // Filter by those that have increased
    let increases = cmp.filter(|(a, b)| if let Some(a) = a { b > a } else { false });

    increases.count()
}

pub fn part2(input: &str) -> usize {
    let input = generator(input).unwrap();

    let offset_1 = iter::once(None).chain(input.clone().into_iter().map(Some));
    let offset_2 = iter::once(None)
        .chain(iter::once(None))
        .chain(input.clone().into_iter().map(Some));

    let windows = izip!(offset_2, offset_1, input).filter_map(|(a, b, c)| {
        if let Some(a) = a {
            if let Some(b) = b {
                Some(a + b + c)
            } else {
                None
            }
        } else {
            None
        }
    });

    let offset_windows = iter::once(None).chain(windows.clone().map(Some));
    let cmp = offset_windows.zip(windows);

    // Filter by those that have increased
    let increases = cmp.filter(|(a, b)| if let Some(a) = a { b > a } else { false });

    increases.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 5);
    }
}
