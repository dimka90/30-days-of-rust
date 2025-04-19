use std::fmt::{self, Display};

use Types::Book;

pub mod Types{

    
    pub struct Book{
       pub books: Vec<String>,
        pub pages: u16,
        pub count: i16,
    }
}

impl fmt::Display for Types::Book{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
       write!(f, "{:?}, {}", self.books, self.pages)
    }
}

impl  Iterator for Book {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <  self.books.len() as i16{
        let result = self.books[self.count as usize].clone();
        self.count+=1;
        return  Some(result);
        }
        None

    }
}