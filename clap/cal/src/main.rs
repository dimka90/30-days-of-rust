use core::num;
use std::{process::CommandArgs, result};

use clap::{Arg, Command};

fn main() {

    let new_command_line = Command::new("Calculator")
                                    .author("Dimka")
                                    .about("A Calculator cli that perform simple multiplication")
                                    .version("v.0.1")
                                    .arg(
                                        Arg::new("Calculator")
                                        .long("Calculator")
                                        .value_name("first Number")
                                        .num_args(2)
                                        .value_parser(clap::value_parser!(i32))
                                    )
                                .get_matches();

    if let Some(values) =  new_command_line.get_many::<i32>("Calculator"){

        let numbers: Vec<i32> = values.copied().collect();
        let first_number = numbers[0];
        let second_number = numbers[1];
        let result = first_number + second_number;
        println!("Result {:?}", result);


    }
    
}
