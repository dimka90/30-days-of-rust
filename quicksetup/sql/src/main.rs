mod sql;
use crate::sql::alchemy::{Session, SessionMaker};
fn main() {
    let url = "postgress://dimka90@12234/user_db";
    let  session = SessionMaker::new(url);
    let  query = session.query("Students");
    println!("{:?}", query);
}
