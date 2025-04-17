fn main() {
    println!("Hello, world!");

    let user:User<i32> = User {user : String::from("dimka"), line: 1};
    println!("{:?}", user);
}



#[derive(Debug)]
struct User  <T> {

    user: String,
    line: T,
}

fn new() -> User<u16>{

    User<u16> { 
        user: "james".to_string(),
        line: 20,
    }
}