
use std::fs::File;
use std::io::{ Stdin, Write};
use std::io;
struct User{
    name: String,
    age:u64,
}
fn main() {
collect_user_data();

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

    let formated_string = format!("Name: {}, Age {}", user.name, user.age);
    let filename = format!("{}.txt", user.name.clone());
    let _data = match store_user_to_file(&formated_string, &filename){
        Ok(_) => println!("User sucessfully created"),
        Err(err) => println!("Error: An Error Occured {}", err),
    };
}

fn store_user_to_file(user_details: &str, filename: &str) -> std::io::Result<()>{

    let mut file: File = File::create(filename)?;
    file.write_all(user_details.as_bytes())?;
    Ok(())

}