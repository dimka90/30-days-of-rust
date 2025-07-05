use std::process;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Calculator")
        .author("Dimka")
        .about("A Calculator CLI that performs simple arithmetic operations")
        .version("v0.1.0")
        .subcommand(
            Command::new("add")
                .about("Add two numbers")
                .arg(
                    Arg::new("first_number")
                        .value_name("FIRST")
                        .help("First number to add")
                        .required(true)
                        .value_parser(clap::value_parser!(i32)),
                )
                .arg(
                    Arg::new("second_number")
                        .value_name("SECOND")
                        .help("Second number to add")
                        .required(true)
                        .value_parser(clap::value_parser!(i32)),
                )
            )
            .subcommand(
                Command::new("mul")
                .about("Multiple two numbers")
                .arg(
                    Arg::new("first_number")
                    .value_name("FIRST NUMBER")
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                )
                .arg(
                    Arg::new("second_number")
                    .value_name("Second Number ")
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                )
            )
        .get_matches();
    match matches.subcommand() {
        Some(("add", add_matches)) => {
            if let (Some(first), Some(second)) = (
                add_matches.get_one::<i32>("first_number"),
                add_matches.get_one::<i32>("second_number"),
            ) {
                println!("Result: {:?}", first + second);
            } else {
                eprint!("Both input are required");
                process::exit(1);
            }
        }

        Some(("mul", mul_matches)) =>{
            if let (Some(first), Some(second)) = (
                mul_matches.get_one::<i32>("first_number"),
                mul_matches.get_one::<i32>("second_number")
            ){

                let sum = first * second;

                println!("Result: {:?}", sum);
            }

            else{
                eprint!("Invalid input for multiplication");
                process::exit(1);
            }
        },
        _ => println!("Invalid input"),
    }
}
