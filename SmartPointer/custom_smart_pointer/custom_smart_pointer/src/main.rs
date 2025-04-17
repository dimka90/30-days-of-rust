mod types;
use types::types::{Tokos};
fn main() {

    let x=4;

    let value = Tokos::new(x);
    println!("Instance one {:?}", value);
    assert_eq!(4, x);
    assert_eq!(4, *value)
    // assert!(4, "{}", *x);
}
