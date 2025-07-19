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

#[cfg(test)]
mod tests{
    use core::num;

    use super::*;

    #[test]
    fn test_valid_age() {
        let num = parse_age("45").ok().unwrap();

        assert_eq!(num, 45);
    }
    #[test]
    fn test_invalid_age(){
        assert_eq!(parse_age("uii"), Err("Invalid number".into()));
    }
}