use std::string;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = numbers.iter().map(|num| num * num).collect();
    println!("{:?}", squares);

    let ages = vec![12, 45, 18, 7, 33, 16, 21];

    // Use iterators to get only ages 18+
    let adults: Vec<_>= ages.iter().filter(|&&age|  age >= 18).collect();
    println!("{:?}", adults);

    let strings = vec!["42", "not a number", "100", "7", "oops"];

    // Parse to i32, keep only successful parses
    let nums: Vec<i32>= strings
                         .iter()
                         .filter_map(|&item| item.parse::<i32>().ok())
                         .collect();

    //My custom implementation
    let nums: Vec<_>= strings
               .iter()
               .map(|&item| item.parse::<i32>().ok())
               .filter(|item| item.is_some())
               .map(|item| item.unwrap())
               .collect();
    println!("{:?}", nums);
}
