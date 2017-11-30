extern crate bowling;
use bowling::Throw;

#[test]
fn parser_detects_bad_input() {
    assert_eq!("invalid pin amount: 0",
               Throw::try_from_str("").unwrap_err().cause().to_string());
    assert_eq!("invalid pin amount: 11",
               Throw::try_from_str("1 0 1 0 1 0 0 0 0 0 1").unwrap_err().cause().to_string());
    assert_eq!("invalid digit found in string",
               Throw::try_from_str("not a number").unwrap_err().cause().to_string());
}

#[test]
fn parser_converts_valid_input() {
    let input = Throw::try_from_str("1 0 1 0 1 0 1 0 1 0");
    assert_eq!(true, input.is_ok());
}
