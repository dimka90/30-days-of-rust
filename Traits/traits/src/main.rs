mod  types;

use types::Types::Book;
fn main() {

    let book1 = Book{
        title:String::from( "Things fall apart"),
        pages: 300,
    };

    println!("{}", book1);
}