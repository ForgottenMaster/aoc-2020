use {
    super::policy::{Policy, Scheme},
    core::str::FromStr,
    std::io::{Error, ErrorKind, Result},
};

#[derive(Debug)]
pub struct Password {
    _policy: Policy,
    _password: String,
}

impl Password {
    pub fn is_valid(&self, scheme: Scheme) -> bool {
        self._policy.is_valid(&self._password, scheme)
    }
}

impl FromStr for Password {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        if let Some(delimiter_index) = string.find(':') {
            let policy = string[..delimiter_index].parse();
            if policy.is_ok() {
                if delimiter_index < string.len() - 1 {
                    Ok(Self {
                        _policy: policy.unwrap(),
                        _password: string[delimiter_index + 1..].trim().to_string(),
                    })
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "input string should be of the form <min>-<max> <char>: <password>",
                    ))
                }
            } else {
                Err(Error::new(ErrorKind::InvalidData, policy.unwrap_err()))
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "input string should be of the form <min>-<max> <char>: <password>",
            ))
        }
    }
}
