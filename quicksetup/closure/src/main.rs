use std::os::linux::raw::stat;

fn main() {
    let sum = |x: f64, y: f64| x + y;
    let result = sum(9.0, 4.0);
    println!("{result}");

    let multiplier = 5;
    let multiple = |x: i32| x * multiplier;
    let product = multiple(9);
    println!("{product}");

    let mut counter = make_counter(10);
    println!("{}", counter()); // 11
    println!("{}", counter()); // 12
    println!("{}", counter()); // 13

    let mut counter2 = make_counter2();
    let result = counter2.increment();
    println!("{}", result);

    let result = &counter2.increment();
    println!("{}", result);
    println!("{}", counter2.reset());
    println!("{}", counter2.increment())
}
fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}

struct Counter {
    value: i32,
}

fn make_counter2() -> Counter {
    Counter { value: 0 }
}
impl Counter {
    fn increment(&mut self) -> i32 {
        let mut count = self.value;
        let mut result = move || {
            count += 1;
            self.value = count;
            self.value
        };
        result()
    }
    fn reset(&mut self) -> i32 {
        let val = 0;
        let mut result = move || {
            self.value = val;
            self.value
        };
        result()
    }
}
