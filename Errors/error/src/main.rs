mod error;
mod types;

use types::custom_types::Users;

use types::custom_types::User;

fn main() {
    let newUser = User::new("Dimka".to_string(), String::from("1234"));

    let mut users = Users { users: Vec::new() };

    let username = newUser.username.clone();
    let password = newUser.password.clone();
    users.add_new_user(newUser);

     match  users.authenticate(&username, &password){

        Ok(v) => println!(" Result {:?}", v),
        Err(error) => println!("Error: {:?}", error)
     }

    
}
