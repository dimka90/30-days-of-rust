use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

    thread::spawn(|| {
        for i in 1..10{
            println!("Current Index {i}");
        }
    });

    for i in 1..10{
        println!("From the main loop{i}");
        thread::sleep(Duration::from_millis(30));

    }
}
