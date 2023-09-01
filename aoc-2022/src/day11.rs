use std::{cell::RefCell, collections::VecDeque, fmt::Debug};

use itertools::Itertools;

#[derive(Clone)]
pub struct Monkey {
    items: RefCell<VecDeque<u128>>,
    operation: fn(u128) -> u128,
    modulo: u128,
    dst: (usize, usize),
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.items))
    }
}

pub fn generate(_: &str) -> [Monkey; 8] {
    [
        // Monkey 0
        Monkey {
            items: RefCell::new(VecDeque::from([91, 66])),
            operation: |old| old * 13,
            modulo: 19,
            dst: (6, 2),
        },
        // Monkey 1
        Monkey {
            items: RefCell::new(VecDeque::from([78, 97, 59])),
            operation: |old| old + 7,
            modulo: 5,
            dst: (0, 3),
        },
        // Monkey 2
        Monkey {
            items: RefCell::new(VecDeque::from([57, 59, 97, 84, 72, 83, 56, 76])),
            operation: |old| old + 6,
            modulo: 11,
            dst: (5, 7),
        },
        // Monkey 3
        Monkey {
            items: RefCell::new(VecDeque::from([81, 78, 70, 58, 84])),
            operation: |old| old + 5,
            modulo: 17,
            dst: (6, 0),
        },
        // Monkey 4
        Monkey {
            items: RefCell::new(VecDeque::from([60])),
            operation: |old| old + 8,
            modulo: 7,
            dst: (1, 3),
        },
        // Monkey 5
        Monkey {
            items: RefCell::new(VecDeque::from([57, 69, 63, 75, 62, 77, 72])),
            operation: |old| old * 5,
            modulo: 13,
            dst: (7, 4),
        },
        // Monkey 6
        Monkey {
            items: RefCell::new(VecDeque::from([73, 66, 86, 79, 98, 87])),
            operation: |old| old * old,
            modulo: 3,
            dst: (5, 2),
        },
        //Monkey 7
        Monkey {
            items: RefCell::new(VecDeque::from([95, 89, 63, 67])),
            operation: |old| old + 2,
            modulo: 2,
            dst: (1, 4),
        },
    ]
}

pub fn part1(input: &[Monkey]) -> u128 {
    let monkeys = input.to_vec();
    let mut activity = vec![0u128; monkeys.len()];

    for _ in 1..=20 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            while let Some(item) = monkey.items.borrow_mut().pop_front() {
                activity[idx] += 1;

                let new = (monkey.operation)(item) / 3;
                let new_monkey = if new % monkey.modulo == 0 {
                    monkey.dst.0
                } else {
                    monkey.dst.1
                };
                monkeys[new_monkey].items.borrow_mut().push_back(new);

                println!("throwing item with worry level {} to {}", new, new_monkey);
            }
        }
    }

    activity.iter().sorted().rev().take(2).product()
}

pub fn part2(input: &[Monkey]) -> u128 {
    let monkeys = input.to_vec();
    let mut activity = vec![0u128; monkeys.len()];
    let large_modulo: u128 = monkeys.into_iter().map(|monkey| monkey.modulo).product();

    for _ in 1..=10_000 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            while let Some(item) = monkey.items.borrow_mut().pop_front() {
                activity[idx] += 1;

                let new = (monkey.operation)(item) % large_modulo;
                let new_monkey = if new % monkey.modulo == 0 {
                    monkey.dst.0
                } else {
                    monkey.dst.1
                };
                monkeys[new_monkey].items.borrow_mut().push_back(new);

                //println!("throwing item with worry level {} to {}", new, new_monkey);
            }
        }
    }

    dbg!(&activity);

    activity.iter().sorted().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sample_monkeys: [Monkey; 4] = [
            Monkey {
                items: RefCell::new(VecDeque::from([79, 98])),
                operation: |old| old * 19,
                modulo: 23,
                dst: (2, 3),
            },
            Monkey {
                items: RefCell::new(VecDeque::from([54, 65, 75, 74])),
                operation: |old| old + 6,
                modulo: 19,
                dst: (2, 0),
            },
            Monkey {
                items: RefCell::new(VecDeque::from([79, 60, 97])),
                operation: |old| old * old,
                modulo: 13,
                dst: (1, 3),
            },
            Monkey {
                items: RefCell::new(VecDeque::from([74])),
                operation: |old| old + 3,
                modulo: 17,
                dst: (0, 1),
            },
        ];

        assert_eq!(part1(&sample_monkeys), 10605);
    }

    #[test]
    fn test_part2() {
        let sample_monkeys: [Monkey; 4] = [
            Monkey {
                items: RefCell::new(VecDeque::from([79, 98])),
                operation: |old| old * 19,
                modulo: 23,
                dst: (2, 3),
            },
            Monkey {
                items: RefCell::new(VecDeque::from([54, 65, 75, 74])),
                operation: |old| old + 6,
                modulo: 19,
                dst: (2, 0),
            },
            Monkey {
                items: RefCell::new(VecDeque::from([79, 60, 97])),
                operation: |old| old * old,
                modulo: 13,
                dst: (1, 3),
            },
            Monkey {
                items: RefCell::new(VecDeque::from([74])),
                operation: |old| old + 3,
                modulo: 17,
                dst: (0, 1),
            },
        ];

        assert_eq!(part2(&sample_monkeys), 2713310158);
    }
}
