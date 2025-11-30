use std::{ sync::Arc, thread, time::Duration};
fn main() {

    let  shared_value = Arc::new(5);
    println!("Hello, world!");

    let handle = thread::spawn(move || {
        for i in 1..=10 {
            println!("Current Index {shared_value  }");
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..10{
        println!("From the main loop{i}");
        thread::sleep(Duration::from_millis(5));
    }
   let _ =  handle.join();
}

