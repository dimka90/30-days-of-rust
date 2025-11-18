use std::iter::Sum;

#[derive(Debug)]
struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = numbers.iter().map(|num| num * num).collect();
    println!("{:?}", squares);

    let ages = vec![12, 45, 18, 7, 33, 16, 21];

    // Use iterators to get only ages 18+
    let adults: Vec<_> = ages.iter().filter(|&&age| age >= 18).collect();
    println!("{:?}", adults);

    let strings = vec!["42", "not a number", "100", "7", "oops"];

    // Parse to i32, keep only successful parses
    let nums: Vec<i32> = strings
        .iter()
        .filter_map(|&item| item.parse::<i32>().ok())
        .collect();

    //My custom implementation
    let nums: Vec<_> = strings
        .iter()
        .map(|&item| item.parse::<i32>().ok())
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect();
    println!("{:?}", nums);

    let names = vec!["alice", "bob", "charlie"];

    let names_to_upper_case: Vec<_> = names.iter().map(|name| name.to_uppercase()).collect();
    println!("{:?}", names_to_upper_case);

    // // Exercise 4: Filter Then Transform
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let filter_square_numbers: Vec<_> = numbers
        .iter()
        .filter(|&number| number % 2 == 0)
        .map(|num| num * num)
        .collect();
    println!("{:?}", filter_square_numbers);

    let words = vec!["hi", "hello", "world", "a", "rust", "programming"];

    let words_uppercase: Vec<_> = words
        .iter()
        .filter(|word| word.len() > 4)
        .map(|word| word.to_uppercase())
        .collect();
    println!("{:?}", words_uppercase);

    let cart = vec![
        Item {
            name: "Apple".to_string(),
            price: 0.5,
            quantity: 10,
        },
        Item {
            name: "Bread".to_string(),
            price: 2.0,
            quantity: 2,
        },
        Item {
            name: "Milk".to_string(),
            price: 3.5,
            quantity: 1,
        },
    ];

    let price: f64= cart.iter().map(|item| item.price * item.quantity as f64).sum();

    println!("{:?}", price);

    // Given:
    let input = "name:Alice,age:50,city:NYC,invalid,country:USA";
    let result: Vec<(_, _)> = input
        .split(",")
        .filter_map(|input| input.split_once(":"))
        .collect();
    println!("{:?}", result);

    // Given:
let emails = vec![
    "user@example.com",
    "invalid.email",
    "another@test.org",
    "bad@",
    "good@domain.co",
];

let email: Vec<_> = emails.iter().filter_map(|email| email.find("@")).collect();
println!(" Email{:?}", email);

let value = "l5:helloi52ee";

// TODO: Filter valid emails (must contain '@' and '.')
// Expected: ["user@example.com", "another@test.org", "good@domain.co"]
let value : Vec<_> = value.split(":").collect();
println!("{:?}" ,value);

let values = [ 1, 2 , 3, 4, 5, 6].iter();

for num in values{
    println!("{:?}", num);
}  let a = "hello world";
    let b = "hello world";

    println!("a: {:p}", a);
    println!("b: {:p}", b);
}
