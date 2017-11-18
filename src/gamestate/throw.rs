use std::str::FromStr;
use failure::Error;

#[derive(Debug, Fail)]
enum ThrowParseError {
    #[fail(display = "invalid pin amount: {}", pins)]
    InvalidPinAmount { pins: usize },
}

#[derive(Debug, Eq, PartialEq)]
pub enum Throw {
    Regular(u8, Vec<bool>),
    Strike(u8, Vec<bool>),
    Spare(u8, Vec<bool>),
}

impl Throw {
    pub fn try_from_str(line: &str) -> Result<Throw, Error> {
        let mut pin_count = 0;
        let pin_state = line.split_whitespace()
            .map(|s| {
                let num = u8::from_str(s)?;
                if num > 0 {
                    pin_count += 1;
                }
                Ok(num > 0)
            })
            .collect::<Result<Vec<_>, _>>();

        if pin_state.is_err() {
            Err(pin_state.unwrap_err())
        } else if pin_state.as_ref().unwrap().len() != 10 {
            Err(ThrowParseError::InvalidPinAmount { pins: pin_state.unwrap().len() }.into())
        } else {
            Ok(Throw::Regular(pin_count, pin_state.unwrap()))
        }
    }
}
