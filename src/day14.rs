use std::collections::{HashMap, HashSet};
use std::ops::Not;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    Mask { zero: u64, one: u64 },
    Memory { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        if left == "mask" {
            let zero = right.chars().rev().enumerate().fold(0u64, |acc, (i, char)| {
                if char == '0' {
                    acc | (1 << i)
                } else {
                    acc
                }
            });
            let one = right.chars().rev().enumerate().fold(0u64, |acc, (i, char)| {
                if char == '1' {
                    acc | (1 << i)
                } else {
                    acc
                }
            });
            Ok(Instruction::Mask { zero, one })
        } else {
            let address = left
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap();
            let address = address.parse::<u64>().unwrap();
            let value = right.parse::<u64>().unwrap();
            Ok(Instruction::Memory { address, value })
        }
    }
}

#[derive(Debug)]
struct Machine {
    program: Vec<Instruction>,
    pc: usize,
    memory: HashMap<u64, u64>,
    zero_mask: u64,
    one_mask: u64,
}

impl Machine {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            memory: HashMap::new(),
            zero_mask: 0,
            one_mask: 0,
        }
    }

    fn step(&mut self) -> Option<()> {
        if self.pc >= self.program.len() {
            return None;
        }
        match self.program[self.pc] {
            Instruction::Mask { zero, one } => {
                self.zero_mask = zero;
                self.one_mask = one;
            }
            Instruction::Memory { address, mut value } => {
                value |= self.one_mask;
                value &= !self.zero_mask;
                self.memory.insert(address, value);
            }
        };
        self.pc += 1;
        Some(())
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Instruction]) -> u64 {
    let mut machine = Machine::new(input.to_vec());
    while let Some(_) = machine.step() {}
    machine.memory.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    todo!()
}
