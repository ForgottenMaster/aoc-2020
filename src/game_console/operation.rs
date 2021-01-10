use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Accumulate,
    Jump,
    Nop,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim(); // best to be safe.
        match string {
            "acc" => Ok(Operation::Accumulate),
            "jmp" => Ok(Operation::Jump),
            "nop" => Ok(Operation::Nop),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid string '{}' for conversion to Operation", string),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulate() {
        let op = "acc".parse::<Operation>();
        assert!(op.is_ok());
        let op = op.unwrap();
        assert!(op == Operation::Accumulate);
    }

    #[test]
    fn test_jump() {
        let op = "jmp".parse::<Operation>();
        assert!(op.is_ok());
        let op = op.unwrap();
        assert!(op == Operation::Jump);
    }

    #[test]
    fn test_nop() {
        let op = "nop".parse::<Operation>();
        assert!(op.is_ok());
        let op = op.unwrap();
        assert!(op == Operation::Nop);
    }

    #[test]
    fn test_invalid() {
        let op = "foo".parse::<Operation>();
        assert!(op.is_err());
    }
}
