use  std::thread;
use std::time::Duration;
fn main() {
let spawn_thread =   thread::spawn(|| {
    let mut x:i64 = 0;
    for i in  10..100_000_000 {
        x +=i;
    }
});
  
    let mut y:i64 = 0;
    for i in 1..100_000_000{
        y += 1;
    }

    spawn_thread.join().unwrap();
}