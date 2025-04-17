pub mod types {
#[derive(Debug)]
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
pub fn reverse(&self) {

    let mut  current = self;

    let count = 1;
    let mut vector: Vec<i32> = Vec::new();
    while let Self::Cons(value,next ) = current{
        vector.push(*value); 
        current = next;
    }

    println!("Reverse other");
    while let Some(num) = vector.pop(){
      
        print!("{ }", num);
    }

}

pub fn get_pointer_for_stored(&self) {
    let mut  current = self;
    let count = 1;

    while  let Self::Cons(value, next)  = current{

        println!("Pointer address {:p}", &*next);
        // assert_eq!(next, *next);
        current = next;
        
    }
}
}

