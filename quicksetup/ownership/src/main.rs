mod database;

use crate::database::database::{DatabaseConfig, DatabaseConfigBuilder};
fn main() {
    let postgress = DatabaseConfigBuilder::new()
                                            .host(String::from("5000"));

    println!("{:?}", postgress);
}
