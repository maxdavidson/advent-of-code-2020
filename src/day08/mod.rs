use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

#[derive(Debug)]
enum InstructionError {
    InvalidOpcode(String),
    InvalidValue(std::num::ParseIntError),
}

impl std::str::FromStr for Instruction {
    type Err = InstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s[4..]
            .trim_start_matches('+')
            .parse()
            .map_err(InstructionError::InvalidValue)?;
        match &s[0..3] {
            "nop" => Ok(Instruction::Nop(val)),
            "acc" => Ok(Instruction::Acc(val)),
            "jmp" => Ok(Instruction::Jmp(val)),
            opcode => Err(InstructionError::InvalidOpcode(opcode.into())),
        }
    }
}

struct CPU {
    program: Box<[Instruction]>,
    visited: HashSet<usize>,
    pc: usize,
    acc: i64,
}

enum CPUState {
    Running,
    Looping(i64),
    Terminated(i64),
}

impl CPU {
    pub fn from(program: impl Into<Box<[Instruction]>>) -> Self {
        CPU {
            program: program.into(),
            visited: HashSet::new(),
            pc: 0,
            acc: 0,
        }
    }

    pub fn step(&mut self) -> CPUState {
        if let Some(instr) = self.program.get(self.pc) {
            if !self.visited.insert(self.pc) {
                return CPUState::Looping(self.acc);
            }

            match instr {
                Instruction::Nop(_) => {
                    self.pc += 1;
                }
                Instruction::Acc(val) => {
                    self.acc += val;
                    self.pc += 1;
                }
                Instruction::Jmp(val) => {
                    self.pc = (self.pc as i64 + val) as usize;
                }
            }

            CPUState::Running
        } else {
            CPUState::Terminated(self.acc)
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let program: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut cpu = CPU::from(program);

    loop {
        match cpu.step() {
            CPUState::Running => {}
            CPUState::Terminated(_) => panic!("Shouldn't terminate!"),
            CPUState::Looping(val) => break val,
        }
    }
}

pub fn part2(input: &str) -> i64 {
    let program: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();

    program
        .iter()
        .enumerate()
        .find_map(|(index, instruction)| {
            let instruction = match *instruction {
                Instruction::Nop(val) => Instruction::Jmp(val),
                Instruction::Jmp(val) => Instruction::Nop(val),
                _ => return None,
            };

            let mut program = program.clone();
            program[index] = instruction;

            let mut cpu = CPU::from(program);

            loop {
                match cpu.step() {
                    CPUState::Running => {}
                    CPUState::Terminated(val) => break Some(val),
                    CPUState::Looping(_) => break None,
                }
            }
        })
        .expect("No solution found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 5);
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 8);
        assert_eq!(part2(INPUT), 1319);
    }
}
