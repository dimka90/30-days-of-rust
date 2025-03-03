
use std::fs::{File, OpenOptions};
use std::io::{ Read, Stdin, Write};
use std::io;
struct User{
    name: String,
    age:u64,
}
impl User {

    fn get_user_details(&self) ->u64{
        return self.age
    }
    
}
fn main() {
collect_user_data();
read_users_from_file();

let user = User{
    name: String::from("Jesus"),
    age: 30,   
};


println!("Age {}",user.get_user_details());
println!("{}", User::get_user_details(&user));
}


fn  collect_user_data(){
    let mut username = String::new();
    let  mut userage = String::new();
    let standard_input:Stdin = io::stdin();
    println!("Enter your details");
    println!("Name: ");
    let _read_line = standard_input.read_line(&mut username).expect("File to read user Name");
    println!("Age: ");
    let _read_line = standard_input.read_line(&mut userage).expect("File to read user age");
    // Remove newline character from the user input from the keyboard
    let age: u64= userage.trim().parse().expect("Fail to pass it");
    create_user(&username, age);

}

fn  create_user( username: &str, age : u64){
    let user: User = User{
        name: (*username).to_string().trim().to_string(),
        age,
    };
    conver_to_string(&user);
}

fn conver_to_string(user: &User){
    // This function convert user struct into a  string format for storage in file
    let formated_string = format!("Name: {}, Age {}\n", user.name, user.age);
    // let filename = format!("{}.txt", user.name.clone());
    let _data = match store_user_to_file(&formated_string){
        Ok(_) => println!("User sucessfully created"),
        Err(err) => println!("Error: An Error Occured {}", err),
    };
}

fn store_user_to_file(user_details: &str) -> std::io::Result<()>{
    // Using file OpenOptions to either create a file if none-exist,
    // Open and append to a file
    let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("blob.txt")
            .expect("Error: File Error");

    file.write_all(user_details.as_bytes())?;
    Ok(())

}

fn  read_users_from_file(){

    // open file
    let mut  file = File::open("blob.txt").unwrap();
    let mut user_datails = String::new();
    // read user content from the file line by line
    file.read_to_string(&mut user_datails).expect("Error");

    println!("================");
    println!("User Details");

    for lines in user_datails.lines(){

        println!("{}", lines)
    }

}

