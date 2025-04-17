
mod types;

use types::types::{User,EmptyUser, UserofTypeTulple, List};

use List::{Cons,Empty };
fn main() {

    let userName = User{name: String::from("Dimka"), age: 90};

    let link_lisk = List::Cons(1, Box::new(List::Cons(3, Box::new(List::Cons(4, Box::new(List::Empty))))));

    let a = Cons(1, Box::new(Cons(2, Box::new(List::Empty))));

    let b =Cons(1,Box::new(Cons(2, Box::new(a))));
    let c = Box::nons(2, b));

printAddress(&link_lisk);


}


fn printAddress(linkedlist: &List){
    let mut current = linkedlist;

    while let List::Cons(value, next) = current {
        println!("Value : {}  Address: {:p}", value , &**next);

        current = next;
    }
}