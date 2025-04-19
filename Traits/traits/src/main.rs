mod  types;
use std::{fmt::Display, vec};
use types::Types::Book;
fn main() {

    let mut book1 = Book{
        books: vec![String::from("Things fall apart"), 
        String::from("Invinsible"),
        String::from("Think and grow rich")],
        pages: 300,
        count: 0,
    };

    println!("{}", book1);
    let word = "Hello";
    print_twice(&word);

    while let Some(book)= book1.next() {
        println!("{book}")
}
}

fn print_twice<T: Display>(data: &T){

    for i in 1..=2{
        println!("{}", data)
    }
}