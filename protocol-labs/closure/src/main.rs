use core::num;
use std::vec;

fn main() {
    // Create a closure that takes two numbers and returns their sum
    let sum = |x: i32, y: i32| x+y ; 
    let add = sum(3, 4);

    let multiplier = 5;
    // Create a closure that multiplies its input by 'multiplier'

    let mul  = |x: i32| x * multiplier;
    let result = mul(2);
    println!("Result {result}");

    let mut add_5 = make_adder(5);
    assert_eq!(add_5(10), 15);
    assert_eq!(add_5(20), 25);

    // Given a Vec<i32>:
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let  result: i32 = numbers
                      .iter()
                      .filter( |num| *num %2 == 0)
                      .map(|num| num  * num)
                      .sum();

println!("Result {result}");
// Use closures to (in ONE chain):
// 1. Filter out odd numbers
// 2. Square the remaining numbers  
// 3. Sum them all

// Expected result: 2² + 4² + 6² + 8² + 10² = 220
let mut nums: Vec<i32> = vec![1, 2, 3, 4];
let double = |x| x *2;
transform_in_place(&mut nums, double);
}


// Write a function that returns a closure
// The closure should add a specific value to its input
fn make_adder(n: i32) -> impl  Fn(i32) -> i32{
     move |x| x +  n
}

fn transform_in_place<F>(vec: &mut Vec<i32>, mut f: F)
where
    F: FnMut(i32) -> i32
{
    for num in vec.iter_mut(){
       *num = f(*num);
    }
}
#[cfg(test)]
mod test{
use super::*;

#[test]
fn test_sum(){
    let sum = |x: i32, y: i32| x+y ; 
let add = sum(4, 5);

assert_eq!(add, 9);
assert_eq!(sum(9,0,), 9);
assert_ne!(sum(2,2), 5);
} 
#[test]
fn test_iterator_chain(){
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let  result: i32 = numbers
                      .iter()
                      .filter( |num| *num %2 == 0)
                      .map(|num| num  * num)
                      .sum();
assert_eq!(result, 220);
assert_ne!(result, 600);
}
#[test]
fn test_make_adder(){
    let result = make_adder(5);
    assert_eq!(result(10), 15);
    assert_eq!(result(30), 35);
    
}
#[test]
fn test_tranform_in_place(){
    let mut numbers = vec![1, 2, 3, 4];
    let add = |x:i32| x *2;
    
    transform_in_place(&mut numbers, add);
    assert_eq!(numbers, vec![2, 4, 6, 8]);
}
}