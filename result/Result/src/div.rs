use core::result::Result;
#[derive(PartialEq, Debug)]
pub enum CalError {
    ParseError,
    DivideByZero,
}

pub fn divide_number(first_number: &str, second_number: &str) -> Result<f64, CalError> {
    if let (Ok(num1), Ok(num2)) = (first_number.parse::<f64>(), second_number.parse::<f64>()) {
        if num2 == 0.0 {
            return Err(CalError::DivideByZero);
        } else {
            Ok(num1 / num2)
        }
    } else {
        Err(CalError::ParseError)
    }
}
#[cfg(test)]
mod test{
use super::*;

#[test]
fn test_division_valid_values() {
    assert_eq!(divide_number("4", "2").ok().unwrap(), 2.0);
}
#[test]
fn test_division_by_zero(){
    assert_eq!(divide_number("1", "0"), Err(CalError::DivideByZero));
}
#[test]
fn test_division_by_string() {
    assert_eq!(divide_number("James", "Dadin"), Err(CalError::ParseError));
}
}