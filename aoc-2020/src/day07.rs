use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
//use recap::Recap;
use regex::Regex;
use serde::Deserialize;
use std::{str::FromStr, collections::HashSet};

#[derive(Debug, Deserialize, Hash, PartialEq)]
//#[recap(regex = r#"(?x)(?P<modifier>.+) (?P<color>.+) bags?"#)]
pub struct Bag {
    modifier: String,
    color: String
}

#[derive(Debug, Deserialize, Hash, PartialEq)]
//#[recap(regex = r#"(?x)(?P<modifier>.+) (?P<color>.+) bags?"#)]
pub struct ManyBags {
    number: u32,
    bag: Bag
}

#[derive(Debug, Deserialize, Hash, PartialEq)]
//#[recap(regex = r#"(?x)(?P<outer>.+) contain ((?P<color>.+)(, |.)?)+"#)]
pub struct BaggageRule {
    outer: String,
    inner: Vec<String>
}

//#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn containing_bags<'a>(rules: &'a str, bag_type: &str) -> Result<HashSet<&'a str>> {
    let mut containing_bags = HashSet::new();
    
    let mut current_bags = direct_containing_bags(rules, bag_type)?;

    current_bags = current_bags.into_iter().flat_map(|b| direct_containing_bags(rules, bag_type));
    

    Ok(containing_bags)
}

pub fn direct_containing_bags<'a>(rules: &'a str, bag_type: &str) -> Result<HashSet<&'a str>> {
    let pattern = Regex::new(&format!(r"(.+) bags contain .+ {}", bag_type))?;

    Ok(pattern
        .captures_iter(rules)
        .map(|c| c.get(1).map(|m| m.as_str()))
        .collect::<Option<HashSet<&str>>>()
        .unwrap_or_default())
}

#[aoc(day7, part1)]
pub fn part_1(input: &str) -> Result<usize> {
    let containing = containing_bags(input, "shiny gold")?;
    dbg!(containing);

    Ok(input.len())
}

#[aoc(day7, part2)]
pub fn part_2(input: &str) -> usize {
  input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str =
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    /*#[test]
    fn test_parse() -> Result<()> {
        assert_eq!(parse(SAMPLE), vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags."
        ]);

        Ok(())
    }*/

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part_1(&SAMPLE)?, 1);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part_2(&SAMPLE), 1);
        Ok(())
    }
}
