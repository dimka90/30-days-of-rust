use std::ops::Deref;

pub mod types{

    #[derive(Debug)]
    pub struct Tokos<T>(pub T);
}


impl<T> types::Tokos<T>{

   pub  fn new(value: T) ->types::Tokos<T> {
        types::Tokos(value)
    }

}

impl<T> Deref  for types::Tokos<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}