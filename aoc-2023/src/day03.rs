use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn part1(input: &str) -> u64 {
    let columns = input.char_indices().find(|(_, c)| *c == '\n').unwrap().0;
    let rows = input.lines().count();

    let mut part_numbers : Vec<u64> = Vec::new();
    let mut number: String = String::new();
    let mut part_number = false;
    for (i, c) in input.char_indices() {
        match c {
            '0'..='9' => {
                number.push(c);

                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        let x = Ord::clamp((i % (columns+1)) as i32 + dx, 0, columns as i32) as usize;
                        let y = Ord::clamp((i / (columns+1)) as i32 + dy, 0, rows as i32) as usize;
                        
                        // +1 to include the newline character
                        if let Some(value) = input.chars().nth(y * (columns+1) + x) {
                            if value.is_ascii_punctuation() && value != '.' {
                                part_number = true;
                            }
                        }
                    }
                }
            },
            _ => {
                if !number.is_empty() && part_number {
                    part_numbers.push(number.parse().unwrap())
                }

                number.clear();
                part_number = false;
            }
        }
    }

    if !number.is_empty() && part_number {
        part_numbers.push(number.parse().unwrap())
    }

    part_numbers.into_iter().sum()
}

pub fn part2(input: &str) -> u64 {
    let columns = input.char_indices().find(|(_, c)| *c == '\n').unwrap().0;
    let rows = input.lines().count();

    let mut part_numbers: Vec<(u64, HashSet<usize>)> = Vec::new();
    let mut number: String = String::new();
    let mut gears: HashSet<usize> = HashSet::new();
    for (i, c) in input.char_indices() {
        match c {
            '0'..='9' => {
                number.push(c);

                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        let x = Ord::clamp((i % (columns+1)) as i32 + dx, 0, columns as i32) as usize;
                        let y = Ord::clamp((i / (columns+1)) as i32 + dy, 0, rows as i32) as usize;
                        
                        // +1 to include the newline character
                        let index = y * (columns+1) + x;
                        if let Some(value) = input.chars().nth(index) {
                            if value == '*' {
                                gears.insert(index);
                            }
                        }
                    }
                }
            },
            _ => {
                if !number.is_empty() && !gears.is_empty() {
                    part_numbers.push((number.parse().unwrap(), gears.clone()));
                }

                number.clear();
                gears.clear();
            }
        }
    }

    if !number.is_empty() && !gears.is_empty() {
        part_numbers.push((number.parse().unwrap(), gears.clone()));
    }

    let mut gear_numbers: HashMap<usize, Vec<u64>> = HashMap::new();
    for (part_number, gears) in part_numbers {
        for gear in gears {
            match gear_numbers.entry(gear) {
                Entry::Occupied(mut e) => { e.get_mut().push(part_number) },
                Entry::Vacant(e) => { e.insert(vec![part_number]); }
            }
        }
    }

    gear_numbers.into_iter().filter(|e| e.1.len() == 2).map(|e| e.1.into_iter().product::<u64>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_1), 467835);
    }
}
