pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn greet(name: &str) -> String{
format!("Good morning {name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn greet_user(){
        let name = "Dimka";
        assert_eq!(greet(name), "Good morning Dimka")
        
    }
    #[test]
    fn fail_greet(){
        let name = "james";
        assert_ne!(greet(name), "Good morning James")
    }
}
