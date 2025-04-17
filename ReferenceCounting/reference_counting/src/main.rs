mod types;
use std::rc::Rc;
use types::types::List;
fn main() {

    let a  = Rc::new(List::Cons(
                        5,Rc::new(List::Cons(
                            10, Rc::new(List::Nil)))));

    let b = List::Cons(2, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));
}
