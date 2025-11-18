use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

    // Test:
    let mut expensive_cache = Cache::new(|| {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        42
    });

    println!("{}", expensive_cache.get_or_init()); // Should print "Calculating..." and take 2 sec
    println!("{}", expensive_cache.get_or_init()); // Should be instant
}
// Build a Cache<T> that:
// - Stores a value of any type T
// - Tracks if it's been calculated
// - Has get_or_init() that takes a closure

struct Cache<T, F>
where
    F: Fn() -> T,
{
    calculation: F,
    value: Option<T>,
}

impl<T, F> Cache<T, F>
where
    F: Fn() -> T,
{
    fn new(calculation: F) -> Self {
        Self {
            calculation,
            value: None,
        }
    }

    fn get_or_init(&mut self) -> &T {
        if self.value.is_none() {
            let result = (self.calculation)();
            self.value = Some(result);
        }
        self.value.as_ref().unwrap()
        // Calculate only once, cache the result
    }
}
