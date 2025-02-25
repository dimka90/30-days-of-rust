use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    // Generatiing random number from 1 to 100
    let random_num: i32 = rand::rng().random_range(1..=100);
    // creatin a growable string to store user type
    let mut user_input = String::new();
    // Storing an instance of Standard Input in a vairable
    let standard_input: io::Stdin = io::stdin();
    // byte_count stored the number of byte read from the standard input(keyboard)
    let byte_count:Result<usize, io::Error> =standard_input.read_line(&mut user_input);
    // Extracting the values from the variant of Result enum
    match  byte_count {
        Ok(_) => {}
        Err(err) => println!("Error {}", err),
    };
    // converting the user_input into a number
    let input:i32= user_input.trim().parse().expect("Error: An error occured");

    // Comparing the number that the user have entered to the 
    // randomly genereted number
    match input.cmp(&random_num) {
        Ordering::Equal => println!("Great, you got the number right"),
        Ordering::Greater => println!("The number is greater, Try again"),
        Ordering::Less => print("Number is less, try again"),
    }
    println!("{}", random_num);
}

fn print(input: &str) {
    println!("{}", input);
}
