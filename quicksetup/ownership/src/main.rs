mod database;

use crate::database::database::{DatabaseConfig, DatabaseConfigBuilder};
fn main() {
    let postgress = DatabaseConfigBuilder::new();
    println!("{:?}", postgress);
}
