extern crate bowling_scorer;
extern crate failure;
use bowling_scorer::*;

#[test]
fn parser_detects_bad_input() {
    assert_eq!("invalid pin amount: 0",
               utilities::parse_input("").unwrap_err().cause().to_string());
    assert_eq!("invalid pin amount: 11",
               utilities::parse_input("1 0 1 0 1 0 0 0 0 0 1").unwrap_err().cause().to_string());
    assert_eq!("invalid digit found in string",
               utilities::parse_input("not a number").unwrap_err().cause().to_string());
}

#[test]
fn parser_converts_valid_input() {
    let input = utilities::parse_input("1 0 1 0 1 0 1 0 1 0");
    assert_eq!(true, input.is_ok());
}
