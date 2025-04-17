use  crate::List::{Cons,Nil};
use std::rc::Rc;
pub  mod types{
    use std::rc::Rc;

    pub enum List{
        Cons(i32, Rc<List>),
        Nil
    }
}


impl Drop for types::List  {
    fn drop(&mut self) {
        println!("I am drop ")
    }
}