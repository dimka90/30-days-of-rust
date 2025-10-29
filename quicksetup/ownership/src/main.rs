mod database;

use crate::database::database::{DatabaseConfig, DatabaseConfigBuilder};
fn main() {
    let postgress = DatabaseConfigBuilder::new()
                                            .host(String::from("5000"))
                                            .username("Dula@1".to_string())
                                            .password("Dula@1234".to_string())
                                            .build();

    println!("{:?}", postgress);
}
