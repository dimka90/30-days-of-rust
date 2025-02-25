fn main() {
    println!("Hello, world!");
    println!("{}", fahrenhiet_to_celsius(celsius_to_fahrenhiet(37.7)));

    println!("{}",fib(4));
}

// C= 9/5(F−32)

fn fahrenhiet_to_celsius(fah_measurement: f64) -> f64 {
    let celsius = (fah_measurement - 32.0) * (5.0 / 9.0);

    return celsius;
}

fn celsius_to_fahrenhiet(cel_measurement: f64) -> f64 {
    let celsius = ((cel_measurement * 9.0) / 5.0) + 32.0;
    return celsius;
}


fn fib(num: i32) -> i32 {
    if num <= 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } else {
        return fib(num - 1) + fib(num - 2);
    }
}


