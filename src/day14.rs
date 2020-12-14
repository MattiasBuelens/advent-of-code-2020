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

fn to_binary_digits(num: u64) -> [u8; SIZE] {
    (0..SIZE)
        .rev()
        .map(|shift| if num & (1 << shift) != 0 { 1 } else { 0 })
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

fn from_binary_digits(digits: [u8; SIZE]) -> u64 {
    digits
        .iter()
        .fold(0u64, |acc, digit| (acc << 1) | ((*digit != 0) as u64))
}

impl Mask {
    fn new() -> Self {
        Mask { bits: ['0'; SIZE] }
    }

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

    fn get_value_part1(&self, mut value: u64) -> u64 {
        value |= self.ones_mask();
        value &= !self.zero_mask();
        value
    }

    fn get_addresses_part2(&self, input_address: u64) -> Vec<u64> {
        let mut addresses = Vec::<[u8; SIZE]>::new();
        addresses.push(to_binary_digits(input_address));
        for i in 0..SIZE {
            match self.bits[i] {
                '0' => {
                    // do nothing
                }
                '1' => {
                    for address in addresses.iter_mut() {
                        address[i] = 1;
                    }
                }
                'X' => {
                    let mut with_ones = addresses.clone();
                    for address in addresses.iter_mut() {
                        address[i] = 0;
                    }
                    for address in with_ones.iter_mut() {
                        address[i] = 1;
                    }
                    addresses.extend(with_ones);
                }
                bit => panic!("invalid mask bit: {}", bit),
            };
        }
        addresses.into_iter().map(from_binary_digits).collect()
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
    mask: Mask,
}

impl Machine {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            memory: HashMap::new(),
            mask: Mask::new(),
        }
    }

    fn step_part1(&mut self) -> Option<()> {
        if self.pc >= self.program.len() {
            return None;
        }
        match self.program[self.pc] {
            Instruction::Mask { mask } => {
                self.mask = mask;
            }
            Instruction::Memory { address, value } => {
                let value = self.mask.get_value_part1(value);
                self.memory.insert(address, value);
            }
        };
        self.pc += 1;
        Some(())
    }

    fn step_part2(&mut self) -> Option<()> {
        if self.pc >= self.program.len() {
            return None;
        }
        match self.program[self.pc] {
            Instruction::Mask { mask } => {
                self.mask = mask;
            }
            Instruction::Memory { address, value } => {
                let addresses = self.mask.get_addresses_part2(address);
                for address in addresses {
                    self.memory.insert(address, value);
                }
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
    while let Some(_) = machine.step_part1() {}
    machine.memory.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &[Instruction]) -> u64 {
    let mut machine = Machine::new(input.to_vec());
    while let Some(_) = machine.step_part2() {}
    machine.memory.values().sum()
}
