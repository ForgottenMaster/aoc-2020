use std::{
    io::{Error, ErrorKind, Result},
    str::FromStr,
};

#[derive(Debug)]
pub struct Policy {
    _min_repetitions: u8,
    _max_repetitions: u8,
    _char: char,
}

pub enum Scheme {
    OccurrenceCount,
    PositionCheck,
}

impl Policy {
    pub fn is_valid(&self, password: &str, scheme: Scheme) -> bool {
        match scheme {
            Scheme::OccurrenceCount => {
                let count = password.chars().filter(|c| c == &self._char).count() as u8;
                count >= self._min_repetitions && count <= self._max_repetitions
            }
            Scheme::PositionCheck => {
                let c1 = password
                    .chars()
                    .nth(self._min_repetitions as usize - 1)
                    .unwrap();
                let c2 = password
                    .chars()
                    .nth(self._max_repetitions as usize - 1)
                    .unwrap();
                (c1 == self._char && c2 != self._char) || (c1 != self._char && c2 == self._char)
            }
        }
    }
}

impl FromStr for Policy {
    type Err = Error;

    fn from_str(mut string: &str) -> Result<Self> {
        string = string.trim();
        if let Some(delimiter_index) = string.find('-') {
            let min_repetitions = string[..delimiter_index].trim().parse();
            string = &string[delimiter_index + 1..].trim();
            if min_repetitions.is_ok() {
                let min_repetitions = min_repetitions.unwrap();
                if let Some(delimiter_index) = string.find(' ') {
                    let max_repetitions = string[..delimiter_index].trim().parse();
                    string = &string[delimiter_index + 1..].trim();
                    if max_repetitions.is_ok() {
                        let max_repetitions = max_repetitions.unwrap();
                        if string.len() == 1 {
                            Ok(Self {
                                _min_repetitions: min_repetitions,
                                _max_repetitions: max_repetitions,
                                _char: string.chars().next().unwrap(),
                            })
                        } else {
                            Err(Error::new(
                                ErrorKind::InvalidData,
                                "Input string should be of the form <min>-<max> <char>",
                            ))
                        }
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidData,
                            max_repetitions.unwrap_err(),
                        ))
                    }
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Input string should be of the form <min>-<max> <char>",
                    ))
                }
            } else {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    min_repetitions.unwrap_err(),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Input string should be of the form <min>-<max> <char>",
            ))
        }
    }
}
