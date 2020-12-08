use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let op = parts.next().unwrap();
        let arg = parts.next().unwrap().parse::<i32>().unwrap();
        Ok(match op {
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            "nop" => Instruction::Nop(arg),
            _ => panic!("unknown op: {}", op),
        })
    }
}

#[derive(Debug)]
struct Machine {
    program: Vec<Instruction>,
    pc: usize,
    acc: i32,
}

impl Machine {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            acc: 0,
        }
    }

    fn acc(&self) -> i32 {
        self.acc
    }

    fn pc(&self) -> usize {
        self.pc
    }

    fn step(&mut self) {
        match self.program[self.pc] {
            Instruction::Acc(arg) => {
                self.acc += arg;
                self.pc += 1;
            }
            Instruction::Jmp(offset) => {
                self.pc = ((self.pc as isize) + (offset as isize)) as usize;
            }
            Instruction::Nop(_) => {
                self.pc += 1;
            }
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn detect_loop(program: &[Instruction]) -> (bool, i32) {
    let mut seen = HashSet::<usize>::new();
    let mut machine = Machine::new(program.to_vec());
    seen.insert(machine.pc());
    while machine.pc() < program.len() {
        machine.step();
        if !seen.insert(machine.pc()) {
            // Already seen this instruction, loop detected
            return (false, machine.acc());
        }
    }
    // Halted
    (true, machine.acc())
}

#[aoc(day8, part1)]
pub fn part1(program: &[Instruction]) -> i32 {
    let (halted, acc) = detect_loop(program);
    assert_eq!(halted, false);
    acc
}

#[aoc(day8, part2)]
pub fn part2(program: &[Instruction]) -> i32 {
    for (i, instruction) in program.iter().enumerate() {
        // Swap jmp with nop
        let modified_instruction = match *instruction {
            Instruction::Jmp(arg) => Instruction::Nop(arg),
            Instruction::Nop(arg) => Instruction::Jmp(arg),
            Instruction::Acc(_) => continue,
        };
        let mut modified_program = program.to_vec();
        modified_program[i] = modified_instruction;
        // Check if modified program terminates
        if let (true, result) = detect_loop(&modified_program) {
            return result;
        }
    }
    todo!()
}
