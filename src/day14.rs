use std::collections::HashMap;
use std::convert::TryInto;
use std::str::FromStr;

const SIZE: usize = 36;

#[derive(Debug, Copy, Clone)]
pub struct Mask {
    bits: [char; SIZE],
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .chars()
            .map(|char| match char {
                '0' | '1' | 'X' => char,
                _ => panic!("invalid bit: {}", char),
            })
            .collect::<Vec<_>>();
        Ok(Mask {
            bits: bits.try_into().unwrap(),
        })
    }
}

impl Mask {
    fn get_mask(&self, digit: char) -> u64 {
        self.bits.iter().enumerate().fold(0u64, |acc, (i, char)| {
            if char == &digit {
                acc | (1 << ((SIZE - 1) - i))
            } else {
                acc
            }
        })
    }

    fn zero_mask(&self) -> u64 {
        self.get_mask('0')
    }

    fn ones_mask(&self) -> u64 {
        self.get_mask('1')
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Mask { mask: Mask },
    Memory { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        if left == "mask" {
            let mask = right.parse::<Mask>().unwrap();
            Ok(Instruction::Mask { mask })
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
            Instruction::Mask { mask } => {
                self.zero_mask = mask.zero_mask();
                self.one_mask = mask.ones_mask();
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
