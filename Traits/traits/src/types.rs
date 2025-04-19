use std::fmt;

pub mod Types{

    pub struct Book{
       pub title: String,
        pub pages: u16
    }
}

impl fmt::Display for Types::Book{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
       write!(f, "{}, {}", self.title, self.pages)
    }
}