use {
    crate::game_console::operation::Operation,
    std::{
        io::{Error, ErrorKind},
        str::FromStr,
    },
};

pub struct Instruction {
    operation: Operation,
    argument: i16,
}

impl Instruction {
    pub fn operation(&self) -> Operation {
        self.operation.clone()
    }

    pub fn argument(&self) -> i16 {
        self.argument
    }

    pub fn apply(&self, acc: i16, pc: usize) -> (i16, usize) {
        match self.operation {
            Operation::Accumulate => (acc + self.argument, pc + 1),
            Operation::Jump => (acc, (pc as i16 + self.argument) as usize),
            Operation::Nop => (acc, pc + 1),
        }
    }

    pub fn try_flip_operation(&mut self) -> bool {
        match self.operation {
            Operation::Accumulate => false,
            Operation::Nop => {
                self.operation = Operation::Jump;
                true
            }
            Operation::Jump => {
                self.operation = Operation::Nop;
                true
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut splits = string.trim().split(" ");
        if let Some(operation) = splits.next() {
            let operation = operation.parse::<Operation>();
            if operation.is_ok() {
                let operation = operation.unwrap();
                if let Some(argument) = splits.next() {
                    let argument = argument.parse::<i16>();
                    if argument.is_ok() {
                        let argument = argument.unwrap();
                        Ok(Self {
                            operation,
                            argument,
                        })
                    } else {
                        Err(Error::new(ErrorKind::InvalidData, argument.unwrap_err()))
                    }
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Argument couldn't be extracted from instruction string '{}'",
                            string
                        ),
                    ))
                }
            } else {
                Err(Error::new(ErrorKind::InvalidData, operation.unwrap_err()))
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Operation couldn't be extracted from instruction string '{}'",
                    string
                ),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_acc() {
        let instruction = "acc +42".parse::<Instruction>().unwrap();
        assert!(instruction.operation() == Operation::Accumulate && instruction.argument() == 42);
    }

    #[test]
    fn test_positive_jmp() {
        let instruction = "jmp +12".parse::<Instruction>().unwrap();
        assert!(instruction.operation() == Operation::Jump && instruction.argument() == 12);
    }

    #[test]
    fn test_positive_nop() {
        let instruction = "nop +24".parse::<Instruction>().unwrap();
        assert!(instruction.operation() == Operation::Nop && instruction.argument() == 24);
    }

    #[test]
    fn test_negative_acc() {
        let instruction = "acc -17".parse::<Instruction>().unwrap();
        assert!(instruction.operation() == Operation::Accumulate && instruction.argument() == -17);
    }

    #[test]
    fn test_negative_jmp() {
        let instruction = "jmp -85".parse::<Instruction>().unwrap();
        assert!(instruction.operation() == Operation::Jump && instruction.argument() == -85);
    }

    #[test]
    fn test_negative_nop() {
        let instruction = "nop -92".parse::<Instruction>().unwrap();
        assert!(instruction.operation() == Operation::Nop && instruction.argument() == -92);
    }

    #[test]
    fn test_invalid_empty() {
        assert!("    ".parse::<Instruction>().is_err());
    }

    #[test]
    fn test_invalid_operation() {
        assert!("foo +92".parse::<Instruction>().is_err());
    }

    #[test]
    fn test_invalid_argument() {
        assert!("acc ?45".parse::<Instruction>().is_err());
    }
}
