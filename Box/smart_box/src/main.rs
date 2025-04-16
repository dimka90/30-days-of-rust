mod types;
use types::types::List;
fn main() {

    let linked_list = List::Cons(1, 
                        Box::new(List::Cons(2, 
                            Box::new(List::Cons(3,
                                 Box::new(List::Nil))))));
linked_list.traverse();
}


