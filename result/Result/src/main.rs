mod  cal;
mod calc;
mod div;
use std::{env::args};

fn main() {
   
//    let args: Vec<String> = args().collect();

//    if args.len() != 4{
//     panic!("Usage: Cargo run <number1> <Operator> <number3>");
//    };
//    println!("args: {:?}", args);

let number = div::divide_number("2", "3").ok().unwrap();

println!("NUmber: {:?}", number);
}
