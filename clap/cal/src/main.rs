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
                                        .value_parser(clap::value_parser!(i32))
                                    )
                                .get_matches();

    println!("My first app {:?}", new_command_line);
}
