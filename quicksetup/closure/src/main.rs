fn main() {
    let sum = |x: f64, y: f64| x + y;
    let result = sum(9.0, 4.0);
    println!("{result}");

    let multiplier = 5;
    let multiple = |x: i32| x * multiplier;
    let product = multiple(9);
    println!("{product}");
}
