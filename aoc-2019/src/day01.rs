use std::num::ParseIntError;
use std::str::FromStr;

pub fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(FromStr::from_str).collect()
}

pub fn part_1(input: &[u32]) -> u32 {
    input.into_iter().map(marginal_fuel_required).sum()
}

pub fn part_2(input: &[u32]) -> u32 {
    input.into_iter().map(fuel_required).sum()
}

fn marginal_fuel_required(weight: &u32) -> u32 {
    (weight / 3).checked_sub(2).unwrap_or_default()
}

fn fuel_required(weight: &u32) -> u32 {
    total_weight(weight) - weight
}

fn total_weight(weight: &u32) -> u32 {
    match marginal_fuel_required(weight) {
        0 => *weight,
        x => weight + total_weight(&x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "12\n14\n1969\n100756";

    #[test]
    fn test_parse() {
        assert_eq!(parse(SAMPLE), Ok(vec![12, 14, 1969, 100756]));
    }

    #[test]
    fn test_marginal_fuel_required() {
        assert_eq!(marginal_fuel_required(&12), 2);
        assert_eq!(marginal_fuel_required(&14), 2);
        assert_eq!(marginal_fuel_required(&1969), 654);
        assert_eq!(marginal_fuel_required(&100756), 33583);
    }

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(&14), 2);
        assert_eq!(fuel_required(&1969), 966);
        assert_eq!(fuel_required(&100756), 50346);
    }

    #[test]
    fn test_part_1() {
        let parsed = parse(SAMPLE).unwrap();
        assert_eq!(part_1(&parsed), 34241);
    }

    #[test]
    fn test_part_2() {
        let parsed = parse(SAMPLE).unwrap();
        assert_eq!(part_2(&parsed), 51316);
    }
}
