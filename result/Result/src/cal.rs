use core::result::Result;
pub fn parse_age(age: &str) -> Result<u8, String>
{
    if let Ok(num) = age.parse::<u8>() {
       Ok(num)
    }
    else {
Err("Invalid number".into())
    }
}