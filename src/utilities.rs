use std::str::FromStr;
use failure::Error;

#[derive(Debug, Fail)]
enum ParseError {
    #[fail(display = "invalid pin amount: {}", pins)]
    InvalidPinAmount { pins: usize },
}

pub fn parse_input(line: &str) -> Result<Vec<bool>, Error> {
    let pin_state = line.split_whitespace()
        .map(|s| {
            let num = u8::from_str(s)?;
            Ok(num > 0)
        })
        .collect::<Result<Vec<_>, _>>();

    if pin_state.is_ok() && pin_state.as_ref().unwrap().len() != 10 {
        return Err(ParseError::InvalidPinAmount { pins: pin_state.unwrap().len() }.into());
    }

    pin_state
}
