use {
    crate::game_console::instruction::Instruction,
    std::{collections::HashSet, io::Error, str::FromStr},
};

pub struct Program {
    instructions: Box<[Instruction]>,
    acc: i16,
    pc: usize,
}

pub enum ProgramResult {
    Termination(i16),
    InfiniteLoop(i16),
}

impl Program {
    pub fn execute_and_reset(&mut self) -> ProgramResult {
        let mut visited = HashSet::with_capacity(self.len());
        let result = loop {
            if !visited.insert(self.pc) {
                break ProgramResult::InfiniteLoop(self.acc);
            }

            if !self.advance() {
                break ProgramResult::Termination(self.acc);
            }
        };
        self.acc = 0;
        self.pc = 0;
        result
    }

    pub fn try_flip_operation(&mut self, pc: usize) -> bool {
        if pc < self.len() {
            self.instructions[pc].try_flip_operation()
        } else {
            false
        }
    }

    pub fn execute_with_flipped_operation_and_reset(&mut self, pc: usize) -> ProgramResult {
        if self.try_flip_operation(pc) {
            let result = self.execute_and_reset();
            self.try_flip_operation(pc);
            result
        } else {
            self.execute_and_reset()
        }
    }

    fn advance(&mut self) -> bool {
        if self.pc < self.instructions.len() {
            let (acc, pc) = self.instructions[self.pc].apply(self.acc, self.pc);
            self.acc = acc;
            self.pc = pc;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn acc(&self) -> i16 {
        self.acc
    }

    pub fn pc(&self) -> usize {
        self.pc
    }
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for line in string.trim().lines() {
            instructions.push(line.parse()?);
        }
        let instructions = instructions.into_boxed_slice();
        let acc = 0;
        let pc = 0;

        Ok(Self {
            instructions,
            acc,
            pc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        const INPUT: &str = "nop +0
                             acc +1
                             jmp +4
                             acc +3
                             jmp -3
                             acc -99
                             acc +1
                             jmp -4
                             acc +6";
        let mut program = INPUT.parse::<Program>().unwrap();
        assert_eq!(program.pc(), 0);
        assert_eq!(program.acc(), 0);

        assert!(program.advance());
        assert_eq!(program.pc(), 1);
        assert_eq!(program.acc(), 0);

        assert!(program.advance());
        assert_eq!(program.pc(), 2);
        assert_eq!(program.acc(), 1);

        assert!(program.advance());
        assert_eq!(program.pc(), 6);
        assert_eq!(program.acc(), 1);

        assert!(program.advance());
        assert_eq!(program.pc(), 7);
        assert_eq!(program.acc(), 2);
    }
}
