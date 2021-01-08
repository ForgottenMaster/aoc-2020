use std::{
    io::{Error, ErrorKind, Result},
    str::FromStr,
};

#[derive(Debug)]
pub struct PassportValidation {
    byr: Option<bool>,
    iyr: Option<bool>,
    eyr: Option<bool>,
    hgt: Option<bool>,
    hcl: Option<bool>,
    ecl: Option<bool>,
    pid: Option<bool>,
}

impl PassportValidation {
    pub fn merge(&mut self, other: Self) {
        if self.byr.is_none() {
            self.byr = other.byr;
        }

        if self.iyr.is_none() {
            self.iyr = other.iyr;
        }

        if self.eyr.is_none() {
            self.eyr = other.eyr;
        }

        if self.hgt.is_none() {
            self.hgt = other.hgt;
        }

        if self.hcl.is_none() {
            self.hcl = other.hcl;
        }

        if self.ecl.is_none() {
            self.ecl = other.ecl;
        }

        if self.pid.is_none() {
            self.pid = other.pid;
        }
    }

    pub fn valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn valid_full(&self) -> bool {
        self.valid()
            && self.byr.unwrap()
            && self.iyr.unwrap()
            && self.eyr.unwrap()
            && self.hgt.unwrap()
            && self.hcl.unwrap()
            && self.ecl.unwrap()
            && self.pid.unwrap()
    }
}

impl Default for PassportValidation {
    fn default() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        }
    }
}

impl FromStr for PassportValidation {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;

        for component in string.trim().split(" ") {
            let component = component.trim();
            let mut subcomponents = component.split(":");
            let key = subcomponents.next();
            if let Some(value) = subcomponents.next() {
                match key {
                    Some("byr") => {
                        byr = Some(validate_byr(value));
                    }
                    Some("iyr") => {
                        iyr = Some(validate_iyr(value));
                    }
                    Some("eyr") => {
                        eyr = Some(validate_eyr(value));
                    }
                    Some("hgt") => {
                        hgt = Some(validate_hgt(value));
                    }
                    Some("hcl") => {
                        hcl = Some(validate_hcl(value));
                    }
                    Some("ecl") => {
                        ecl = Some(validate_ecl(value));
                    }
                    Some("pid") => {
                        pid = Some(validate_pid(value));
                    }
                    Some("cid") => {
                        // ignored
                    }
                    _ => {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid data"));
                    }
                }
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid data"));
            }
        }

        Ok(Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
        })
    }
}

fn validate_byr(input: &str) -> bool {
    if input.len() == 4 {
        if let Ok(value) = input.parse::<u16>() {
            value >= 1920 && value <= 2002
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_iyr(input: &str) -> bool {
    if input.len() == 4 {
        if let Ok(value) = input.parse::<u16>() {
            value >= 2010 && value <= 2020
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_eyr(input: &str) -> bool {
    if input.len() == 4 {
        if let Ok(value) = input.parse::<u16>() {
            value >= 2020 && value <= 2030
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_hgt(input: &str) -> bool {
    if let Some(index) = input.find("cm") {
        if let Ok(value) = &input[..index].parse::<u8>() {
            *value >= 150 && *value <= 193
        } else {
            false
        }
    } else if let Some(index) = input.find("in") {
        if let Ok(value) = &input[..index].parse::<u8>() {
            *value >= 59 && *value <= 76
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_hcl(input: &str) -> bool {
    if input.starts_with("#") {
        input
            .chars()
            .skip(1)
            .filter(|c| (*c >= '0' && *c <= '9') || (*c >= 'a' && *c <= 'f'))
            .count()
            == 6
    } else {
        false
    }
}

fn validate_ecl(input: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input)
}

fn validate_pid(input: &str) -> bool {
    if input.len() == 9 {
        if let Ok(_) = input.parse::<u64>() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use {super::PassportValidation, std::io::Result};

    #[test]
    fn valid_full_2_lines() -> Result<()> {
        const INPUT_LINE_1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd";
        const INPUT_LINE_2: &str = "byr:1937 iyr:2017 cid:147 hgt:183cm";
        let mut passport_validation = INPUT_LINE_1.parse::<PassportValidation>()?;
        passport_validation.merge(INPUT_LINE_2.parse()?);
        assert!(passport_validation.valid());
        Ok(())
    }

    #[test]
    fn invalid_missing_hgt() -> Result<()> {
        const INPUT_LINE_1: &str = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884";
        const INPUT_LINE_2: &str = "hcl:#cfa07d byr:1929";
        let mut passport_validation = INPUT_LINE_1.parse::<PassportValidation>()?;
        passport_validation.merge(INPUT_LINE_2.parse()?);
        assert!(!passport_validation.valid());
        Ok(())
    }

    #[test]
    fn valid_missing_only_cid() -> Result<()> {
        const INPUT_LINE_1: &str = "hcl:#ae17e1 iyr:2013";
        const INPUT_LINE_2: &str = "eyr:2024";
        const INPUT_LINE_3: &str = "ecl:brn pid:760753108 byr:1931";
        const INPUT_LINE_4: &str = "hgt:179cm";
        let mut passport_validation = INPUT_LINE_1.parse::<PassportValidation>()?;
        passport_validation.merge(INPUT_LINE_2.parse()?);
        passport_validation.merge(INPUT_LINE_3.parse()?);
        passport_validation.merge(INPUT_LINE_4.parse()?);
        assert!(passport_validation.valid());
        Ok(())
    }

    #[test]
    fn invalid_missing_cid_and_byr() -> Result<()> {
        const INPUT_LINE_1: &str = "hcl:#cfa07d eyr:2025 pid:166559648";
        const INPUT_LINE_2: &str = "iyr:2011 ecl:brn hgt:59in";
        let mut passport_validation = INPUT_LINE_1.parse::<PassportValidation>()?;
        passport_validation.merge(INPUT_LINE_2.parse()?);
        assert!(!passport_validation.valid());
        Ok(())
    }
}
