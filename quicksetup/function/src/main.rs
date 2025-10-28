use core::num;




// function with numbers
fn add_number(first_number: f64, second_number: f64)  -> f64{

    return first_number + second_number;
}
// functions with looops
fn print_count( number: i32) {
    for i in 1..=number{
        println!(" {i}");
    }
}



fn main() {
    let sum = add_number(4.1, 6.3);
    print!("{}", sum);
    println!("Hello, world!");
}
