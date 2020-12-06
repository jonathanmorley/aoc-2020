use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct Stack {
    memory: Vec<usize>,
    instruction_pointer: usize,
}

impl FromStr for Stack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Stack {
            memory: s
                .split(',')
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
            instruction_pointer: 0,
        })
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.memory.iter().join(","))
    }
}

impl Stack {
    pub fn input(&mut self, noun: usize, verb: usize) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    pub fn run(&mut self) -> Result<&Stack> {
        loop {
            if let None = self.apply()? {
                return Ok(self);
            }
        }
    }

    pub fn output(&self) -> Result<usize> {
        self.read(0)
    }

    fn read(&self, index: usize) -> Result<usize> {
        self.memory
            .get(index)
            .map(|x| *x)
            .ok_or_else(|| anyhow!("Index {} does not exist", index))
    }

    fn instruction(&self) -> Result<Instruction> {
        match self.memory[self.instruction_pointer] {
            1 => Ok(Instruction::Add {
                left_index: self.read(self.instruction_pointer + 1)?,
                right_index: self.read(self.instruction_pointer + 2)?,
                target_index: self.read(self.instruction_pointer + 3)?,
            }),
            2 => Ok(Instruction::Multiply {
                left_index: self.read(self.instruction_pointer + 1)?,
                right_index: self.read(self.instruction_pointer + 2)?,
                target_index: self.read(self.instruction_pointer + 3)?,
            }),
            99 => Ok(Instruction::Halt),
            a => bail!("Invalid opcode {}", a),
        }
    }

    fn apply(&mut self) -> Result<Option<()>> {
        match self.instruction()? {
            Instruction::Add {
                left_index,
                right_index,
                target_index,
            } => {
                self.memory[target_index] = self.read(left_index)? + self.read(right_index)?;
                self.instruction_pointer += 4;

                Ok(Some(()))
            }
            Instruction::Multiply {
                left_index,
                right_index,
                target_index,
            } => {
                self.memory[target_index] = self.read(left_index)? * self.read(right_index)?;
                self.instruction_pointer += 4;

                Ok(Some(()))
            }
            Instruction::Halt => Ok(None),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add {
        left_index: usize,
        right_index: usize,
        target_index: usize,
    },
    Multiply {
        left_index: usize,
        right_index: usize,
        target_index: usize,
    },
    Halt,
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_1: &str = "1,9,10,3,2,3,11,0,99,30,40,50";
    static SAMPLE_2: &str = "1,0,0,0,99";
    static SAMPLE_3: &str = "2,3,0,3,99";
    static SAMPLE_4: &str = "2,4,4,5,99,0";
    static SAMPLE_5: &str = "1,1,1,4,99,5,6,0,99";

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(SAMPLE_1.parse::<Stack>()?.to_string(), SAMPLE_1);
        assert_eq!(SAMPLE_2.parse::<Stack>()?.to_string(), SAMPLE_2);
        assert_eq!(SAMPLE_3.parse::<Stack>()?.to_string(), SAMPLE_3);
        assert_eq!(SAMPLE_4.parse::<Stack>()?.to_string(), SAMPLE_4);
        assert_eq!(SAMPLE_5.parse::<Stack>()?.to_string(), SAMPLE_5);

        Ok(())
    }

    #[test]
    fn test_run() -> Result<()> {
        assert_eq!(
            SAMPLE_1.parse::<Stack>()?.run()?.to_string(),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
        assert_eq!(SAMPLE_2.parse::<Stack>()?.run()?.to_string(), "2,0,0,0,99");
        assert_eq!(SAMPLE_3.parse::<Stack>()?.run()?.to_string(), "2,3,0,6,99");
        assert_eq!(
            SAMPLE_4.parse::<Stack>()?.run()?.to_string(),
            "2,4,4,5,99,9801"
        );
        assert_eq!(
            SAMPLE_5.parse::<Stack>()?.run()?.to_string(),
            "30,1,1,4,2,5,6,0,99"
        );

        Ok(())
    }
}
