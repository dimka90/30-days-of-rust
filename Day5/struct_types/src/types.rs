pub mod types{
#[derive(Debug)]
pub struct User{
pub name : String,
pub age: u16,
}
pub struct EmptyUser;
pub struct UserofTypeTulple(String, u16);


#[derive(Debug)]
pub enum  List{
    
    Empty,
    Cons(i32, Box<List>)
}
}