mod otp;

use crate::otp::otp::generate_otp;
fn main() {
    //Generating 10 random numbers
    for _ in 0..=10{
        let n = generate_otp();
        println!("Otp: {}", n);
    }
    println!("Hello, world!");
}
