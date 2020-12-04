use aoc_runner_derive::aoc;

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use std::{convert::TryInto, fmt};
use std::str::FromStr;

/*#[derive(Debug, Hash, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Halt
}

impl TryFrom<usize> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Multiply),
            99 => Ok(Self::Halt),
            _ => bail!("Unexpected Operator ({})", value)
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "1"),
            Operator::Multiply => write!(f, "2"),
            Operator::Halt => write!(f, "99"),
        }
    }
}*/

pub enum Effect {
    Update {
        index: usize,
        value: usize,
    },
    Halt
}

/*impl TryFrom<Operation> for Effect {
    type Error = anyhow::Error;

    fn try_from(value: Operation) -> Result<Self, Self::Error> {
        if value.operator == 99 {
            return Ok(Self::Halt);
        }

        let left = value.left.ok_or_else(|| anyhow!("Left value not found in {}", value))?;
        let right = value.right.ok_or_else(|| anyhow!("Right value not found in {}", value))?;
        let target = value.target.ok_or_else(|| anyhow!("Target value not found in {}", value))?;

        match value.operator {
            1 => Ok(Self::Update {  index: target, value: left + right }),
            2 =>  Ok(Self::Update {  index: target, value: left * right }),
            _ => bail!("Unexpected Operator ({})", value)
        }
    }
}*/

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Operation {
    operator: usize,
    left: Option<usize>,
    right: Option<usize>,
    target: Option<usize>
}

impl Operation {
    pub fn new(operator: usize, left: Option<usize>, right: Option<usize>, target: Option<usize>) -> Result<Self> {
        Ok(Self {
            operator,
            left,
            right,
            target
        })
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        match index {
            0 => Some(self.operator),
            1 => self.left,
            2 => self.right,
            3 => self.target,
            _ => None
        }
    }

    pub fn update(&mut self, index: usize, value: usize) -> Result<()> {
        match index {
            0 => self.operator = value.try_into()?,
            1 => self.left = Some(value),
            2 => self.right = Some(value),
            3 => self.target = Some(value),
            _ => bail!("Unexpected index ({}), should be <= 3", index)
        };

        Ok(())
    }

    pub fn effect(&self, operations: &Operations) -> Result<Effect> {
        if self.operator == 99 {
            return Ok(Effect::Halt);
        }

        let left_index = self.left.ok_or_else(|| anyhow!("Left index not found in {}", self))?;
        let left = operations.get(left_index).ok_or_else(|| anyhow!("Index {} not found in {}", left_index, operations))?;

        let right_index = self.right.ok_or_else(|| anyhow!("Right index not found in {}", self))?;
        let right = operations.get(right_index).ok_or_else(|| anyhow!("Index {} not found in {}", right_index, operations))?;
        
        let target_index = self.target.ok_or_else(|| anyhow!("Target index not found in {}", self))?;

        match self.operator {
            1 => Ok(Effect::Update {  index: target_index, value: left + right }),
            2 =>  Ok(Effect::Update {  index: target_index, value: left * right }),
            _ => bail!("Unexpected Operator ({})", self.operator)
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.operator)?;

        if let Some(left) = self.left {
            write!(f, ",{}", left)?;
        }

        if let Some(right) = self.right {
            write!(f, ",{}", right)?;
        }

        if let Some(target) = self.target {
            write!(f, ",{}", target)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Operations(Vec<Operation>);

impl Operations {
    fn get(&self, index: usize) -> Option<usize> {
        self.0[index/4].get(index%4)
    }

    fn update(&mut self, index: usize, value: usize) -> Result<()> {
        self.0[index/4].update(index%4, value)
    }

    fn run(&mut self) -> Result<&Operations> {
        for i in 0..self.0.len() {
            match self.0[i].effect(&self)? {
                Effect::Update { index, value } => {                    
                    self.update(index, value)?;
                },
                Effect::Halt => return Ok(self)
            }
        };

        bail!("No halting opcode found")
    }
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().join(","))
    }
}

impl FromStr for Operations {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opcodes = s.split(',').map(FromStr::from_str).collect::<Result<Vec<_>, _>>()?;

        let operations = opcodes
            .chunks(4)
            .map(|chunk| Operation::new(
                chunk[0],
                chunk.get(1).cloned(),
                chunk.get(2).cloned(),
                chunk.get(3).cloned()
            ))
            .collect::<Result<Vec<Operation>>>()?;
        
        Ok(Operations(operations))
    }
}

pub fn part_1(input: &str) -> Result<usize> {
    let mut operations: Operations = input.parse()?;

    operations.update(1, 12)?;
    operations.update(2, 2)?;
    operations.run()?;
    operations.get(0).ok_or_else(|| anyhow!("No value found"))
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
    fn test_operations() -> Result<()> {
        assert_eq!(SAMPLE_1.parse::<Operations>()?.to_string(), SAMPLE_1);
        assert_eq!(SAMPLE_2.parse::<Operations>()?.to_string(), SAMPLE_2);
        assert_eq!(SAMPLE_3.parse::<Operations>()?.to_string(), SAMPLE_3);
        assert_eq!(SAMPLE_4.parse::<Operations>()?.to_string(), SAMPLE_4);
        assert_eq!(SAMPLE_5.parse::<Operations>()?.to_string(), SAMPLE_5);

        Ok(())
    }

    #[test]
    fn test_run() -> Result<()> {
        assert_eq!(SAMPLE_1.parse::<Operations>()?.run()?.to_string(), "3500,9,10,70,2,3,11,0,99,30,40,50");
        assert_eq!(SAMPLE_2.parse::<Operations>()?.run()?.to_string(), "2,0,0,0,99");
        assert_eq!(SAMPLE_3.parse::<Operations>()?.run()?.to_string(), "2,3,0,6,99");
        assert_eq!(SAMPLE_4.parse::<Operations>()?.run()?.to_string(), "2,4,4,5,99,9801");
        assert_eq!(SAMPLE_5.parse::<Operations>()?.run()?.to_string(), "30,1,1,4,2,5,6,0,99");

        Ok(())
    }
}
