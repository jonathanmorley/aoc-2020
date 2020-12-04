use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{str::FromStr, convert::TryFrom};

use aoc_runner_derive::aoc;

#[derive(Debug, Hash, PartialEq)]
pub struct Credentials {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
    raw: String
}

impl FromStr for Credentials {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BIRTH_YEAR: Regex = Regex::new(r"byr:(\S+)").unwrap();
            static ref ISSUE_YEAR: Regex = Regex::new(r"iyr:(\S+)").unwrap();
            static ref EXPIRATION_YEAR: Regex = Regex::new(r"eyr:(\S+)").unwrap();
            static ref HEIGHT: Regex = Regex::new(r"hgt:(\S+)").unwrap();
            static ref HAIR_COLOR: Regex = Regex::new(r"(?s)hcl:(\S+)").unwrap();
            static ref EYE_COLOR: Regex = Regex::new(r"ecl:(\S+)").unwrap();
            static ref PASSPORT_ID: Regex = Regex::new(r"pid:(\S+)").unwrap();
            static ref COUNTRY_ID: Regex = Regex::new(r"cid:(\S+)").unwrap();
        }

        fn find_match<'a>(s: &'a str, regex: &Regex) -> Option<&'a str> {
            regex.captures(s)?.get(1).map(|x| x.as_str())
        }

        let birth_year = find_match(s, &BIRTH_YEAR)
            .ok_or_else(|| anyhow!("No birth year found in {}", s))?
            .to_owned();
        let issue_year = find_match(s, &ISSUE_YEAR)
            .ok_or_else(|| anyhow!("No issue year found in {}", s))?
            .to_owned();
        let expiration_year = find_match(s, &EXPIRATION_YEAR)
            .ok_or_else(|| anyhow!("No expiration year found in {}", s))?
            .to_owned();
        let height = find_match(s, &HEIGHT)
            .ok_or_else(|| anyhow!("No height found in {}", s))?
            .to_owned();
        let hair_color = find_match(s, &HAIR_COLOR)
            .ok_or_else(|| anyhow!("No hair color found in {}", s))?
            .to_owned();
        let eye_color = find_match(s, &EYE_COLOR)
            .ok_or_else(|| anyhow!("No eye color found in {}", s))?
            .to_owned();
        let passport_id = find_match(s, &PASSPORT_ID)
            .ok_or_else(|| anyhow!("No passport ID found in {}", s))?
            .to_owned();
        let country_id = find_match(s, &COUNTRY_ID).map(String::from);

        Ok(Credentials {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
            raw: s.into()
        })
    }
}

#[derive(Debug, Hash, PartialEq)]
enum Height {
    Centimetres(u32),
    Inches(u32)
}

impl FromStr for Height {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("cm") {
            return Ok(Self::Centimetres(s[..s.len() - 2].parse()?))
        }

        if s.ends_with("in") {
            return Ok(Self::Inches(s[..s.len() - 2].parse()?))
        }

        bail!("Invalid height ({})", s)
    }
}

#[derive(Debug)]
struct ValidatedCredentials {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: Height,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
    raw: String
}

impl TryFrom<Credentials> for ValidatedCredentials {
    type Error = anyhow::Error;

    fn try_from(value: Credentials) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref YEAR: Regex = Regex::new(r"^\d{4}$").unwrap();
            static ref HAIR_COLOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref EYE_COLOR: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PASSPORT_ID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        if !YEAR.is_match(&value.birth_year) {
            bail!("Invalid birth year ({})", value.birth_year);
        }

        let birth_year = value.birth_year.parse()?;
        if !(1920..=2002).contains(&birth_year) {
            bail!("Invalid birth year ({})", birth_year);
        }

        if !YEAR.is_match(&value.issue_year) {
            bail!("Invalid issue year ({})", value.issue_year);
        }

        let issue_year = value.issue_year.parse()?;
        if !(2010..=2020).contains(&issue_year) {
            bail!("Invalid issue year ({})", issue_year);
        }

        if !YEAR.is_match(&value.expiration_year) {
            bail!("Invalid expiration year ({})", value.expiration_year);
        }

        let expiration_year = value.expiration_year.parse()?;
        if !(2020..=2030).contains(&expiration_year) {
            bail!("Invalid expiration year ({})", expiration_year);
        }

        let height = value.height.parse()?;
        match height {
            Height::Centimetres(h) => {
                if !(150..=193).contains(&h) {
                    bail!("Invalid height ({}cm)", h);
                }
            },
            Height::Inches(h) => {
                if !(59..=76).contains(&h) {
                    bail!("Invalid height ({}in)", h);
                }
            },
        }

        if !HAIR_COLOR.is_match(&value.hair_color) {
            bail!("Invalid hair color ({})", value.hair_color);
        }

        if !EYE_COLOR.is_match(&value.eye_color) {
            bail!("Invalid eye color ({})", value.eye_color);
        }

        if !PASSPORT_ID.is_match(&value.passport_id) {
            bail!("Invalid passport ID ({})", value.passport_id);
        }

        Ok(ValidatedCredentials {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color: value.hair_color,
            eye_color: value.eye_color,
            passport_id: value.passport_id,
            country_id: value.country_id,
            raw: value.raw
        })
    }
}

pub fn parse(input: &str) -> impl Iterator<Item=Credentials> + '_ {
    input
        .split("\n\n")
        .map(|s| s.parse())
        .filter_map(Result::ok)
}

#[aoc(day4, part1)]
pub fn part_1(input: &str) -> usize {
    parse(input).count()
}

#[aoc(day4, part2)]
pub fn part_2(input: &str) -> usize {
    parse(input)
        .map(ValidatedCredentials::try_from)
        .filter_map(Result::ok)
        //.inspect(|c| { dbg!(c); })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static SAMPLE_2: &str = 
"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(
            parse(SAMPLE_1).collect::<Vec<_>>(),
            vec![
                Credentials {
                    birth_year: "1937".into(),
                    issue_year: "2017".into(),
                    expiration_year: "2020".into(),
                    height: "183cm".into(),
                    hair_color: "#fffffd".into(),
                    eye_color: "gry".into(),
                    passport_id: "860033327".into(),
                    country_id: Some("147".into()),
                    raw: "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".into()
                },
                Credentials {
                    birth_year: "1931".into(),
                    issue_year: "2013".into(),
                    expiration_year: "2024".into(),
                    height: "179cm".into(),
                    hair_color: "#ae17e1".into(),
                    eye_color: "brn".into(),
                    passport_id: "760753108".into(),
                    country_id: None,
                    raw: "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm".into()
                }
            ]
        );
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(part_1(&SAMPLE_1), 2);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(part_2(&SAMPLE_1), 2);
        assert_eq!(part_2(&SAMPLE_2), 4);
        Ok(())
    }
}
