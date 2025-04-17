use std::ops::Deref;
pub mod types{


    #[derive(Debug)]
    pub struct Tokos<T>(pub T);
}
impl<T> Deref for types::Tokos<T>{
    type  Target = T;
    fn deref(&self) ->&Self::Target{
       &self.0

    }

}

impl<T>  Drop for types::Tokos<T> {
    fn drop(&mut self) {
        
        println!("I am been drop ");
    }
}
