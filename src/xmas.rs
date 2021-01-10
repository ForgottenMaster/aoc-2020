use {
    crate::cartesian,
    std::{
        io::{Error, ErrorKind},
        ops::Index,
        str::FromStr,
    },
};

pub struct Preamble {
    value: Box<[u128]>,
}

impl Preamble {
    fn valid(&self, number: u128) -> bool {
        cartesian!(self.value.iter(), self.value.iter())
            .filter(|(v1, v2)| (v1 != v2) && (**v1 + **v2) == number)
            .take(1)
            .count()
            > 0
    }

    fn insert(&mut self, number: u128) {
        for i in 0..(self.value.len() - 1) {
            self.value[i] = self.value[i + 1];
        }
        self.value[self.value.len() - 1] = number;
    }
}

#[derive(Clone)]
pub struct Message {
    value: Box<[u128]>,
}

impl Message {
    fn len(&self) -> usize {
        self.value.len()
    }

    pub fn find_sequence_totalling(&self, number: u128) -> &[u128] {
        for i in 0..self.value.len() {
            let mut running_total = self.value[i];
            let mut j = i + 1;

            while running_total < number && j < self.value.len() {
                running_total += self.value[j];
                j += 1;
            }

            if running_total == number {
                return &self.value[i..j];
            }
        }
        &[]
    }
}

impl Index<usize> for Message {
    type Output = u128;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.value[idx]
    }
}

fn parse_to_number_sequence(string: &str) -> Result<Box<[u128]>, Error> {
    let mut numbers = Vec::new();
    for line in string.trim().lines() {
        match line.trim().parse() {
            Ok(number) => numbers.push(number),
            Err(error) => return Err(Error::new(ErrorKind::InvalidData, error)),
        }
    }
    Ok(numbers.into_boxed_slice())
}

impl FromStr for Preamble {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: parse_to_number_sequence(string)?,
        })
    }
}

impl FromStr for Message {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: parse_to_number_sequence(string)?,
        })
    }
}

pub struct XMAS {
    preamble: Preamble,
    message: Message,
    index: usize,
}

impl XMAS {
    pub fn new(preamble: Preamble, message: Message) -> Self {
        Self {
            preamble,
            message,
            index: 0,
        }
    }
}

impl Iterator for XMAS {
    type Item = (u128, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.message.len() {
            let number = self.message[self.index];
            let valid = self.preamble.valid(number);
            self.index += 1;
            self.preamble.insert(number);
            Some((number, valid))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_preamble() {
        const INPUT: &str = "35
                             20
                             15
                             25
                             47";
        let preamble = INPUT.parse::<Preamble>().unwrap();
        assert_eq!(preamble.value.len(), 5);
        assert_eq!(preamble.value[0], 35);
        assert_eq!(preamble.value[4], 47);
    }

    #[test]
    fn test_parse_message() {
        const INPUT: &str = "40
                             62
                             55
                             65
                             95
                             102
                             117
                             150
                             182
                             127
                             219
                             299
                             277
                             309
                             576";
        let message = INPUT.parse::<Message>().unwrap();
        assert_eq!(message.value.len(), 15);
        assert_eq!(message.value[0], 40);
        assert_eq!(message.value[14], 576);
    }

    #[test]
    fn test_message_check() {
        const PREAMBLE: &str = "35
                                20
                                15
                                25
                                47";
        const MESSAGE: &str = "40
                               62
                               55
                               65
                               95
                               102
                               117
                               150
                               182
                               127
                               219
                               299
                               277
                               309
                               576";
        let preamble = PREAMBLE.parse::<Preamble>().unwrap();
        let message = MESSAGE.parse::<Message>().unwrap();
        let mut xmas = XMAS::new(preamble, message);
        for _ in 0..9 {
            assert_eq!(xmas.next().unwrap().1, true);
        }
        assert_eq!(xmas.next().unwrap(), (127, false));
    }
}
