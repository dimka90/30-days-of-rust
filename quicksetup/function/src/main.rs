fn max(a: i32, b: i32) -> i32{
    if a > b{
        return  a;
    }
    return  b;
}

// function with numbers
fn add_number(first_number: f64, second_number: f64) -> f64 {
    return first_number + second_number;
}
// functions with looops
fn print_count(number: i32) {
    for i in 1..=number {
        println!("{i}");
    }
}
fn main() {
    let sum = add_number(4.1, 6.3);
    print_count(10);
    println!("Max : {}", max(10, 100));
    print!("{}", sum);
}
