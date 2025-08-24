use rand::prelude::*;

pub fn generate_otp() -> String{
let mut rng = rand::rng();
let n = rng.random_range(100000..=900000);
return n.to_string();
}