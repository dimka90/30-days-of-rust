use clap::{Arg, Command};

fn main() {

    let new_command_line = Command::new("Calculator")
                                    .author("Dimka")
                                    .about("A Calculator cli that perform simple multiplication")
                                    .version("v.0.1")
                                    .subcommand(
                                        Command::new("add")
                                        .about("A function that adds to two numbers")
                                    .arg(
                                        Arg::new("numbers")
                                        .long("cal")
                                        .value_name("first Number")
                                        .num_args(2)
                                        .required(true)
                                        .value_parser(clap::value_parser!(i32))
                                    ),
                                )
                                .get_matches();

    if let Some(values) =  new_command_line.get_many::<i32>("numbers"){

        let numbers: Vec<i32> = values.copied().collect();
        let result = numbers[1]+ numbers[0];
        println!("Result {:?}", result);


    }
    
}
