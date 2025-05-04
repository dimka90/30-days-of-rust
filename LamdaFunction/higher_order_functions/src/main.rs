

pub  mod  lambda;

use crate::lambda::lambda::custom_filter;
fn main() {
    println!("Hello, world!");

    let numbers = vec![1, 2, 3, 4, 5];
    let evens = custom_filter(numbers, |x| x % 2 == 0);
    println!("{:?}", evens); 

}
