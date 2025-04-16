pub mod types {

pub enum List{
Cons(i32, Box<List>),
Nil
}
}

impl  types::List {



pub fn traverse(&self) {

    let mut current = self;
    let mut count = 1;
    while let Self::Cons(value, next) = current{
        
        println!("Value at {}:  {} ",count, value );
        count +=1;
        current = next;
    }
}

}

