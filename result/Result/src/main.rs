mod  cal;
fn main() {
    match cal::parse_age("Err") {
        Ok(val) => println!("Value: {:?}", val),
        Err(err) => println!("Error: {:?}", err)
    }
}
