// use std::collections::HashMap;
// #[derive(Debug, Clone)]
// struct User{
//     username: String,
//     age: u16,
// }
// #[derive(Debug, Clone)]
// struct UserManager{
//     all_users: HashMap<u16, User>
// }


// impl  UserManager  {
//     fn new() -> Self{
//         Self{
//             all_users: HashMap::new(),
//         }
//     }
// }
// impl  User  {
//     fn new(username: String,  age: u16) -> Self{
//         Self{
//             username,
//             age,
//         }
//     }
// }

// fn addUser(user: &mut User) -> &HashMap<u16, &User>{

//     let mut users: HashMap<u16, &User> =   HashMap::new();
//         users.insert( user.age, &user);

//         &users
// }

use core::num;
use std::vec;


fn main() {

    // let user = User::new(String::from("Dimka"), 27);
    // let user2 = User::new(String::from("Dimka"), 97);

    // let mut all_users = UserManager::new();
    // all_users.all_users.insert(user.age, user.clone());
    // all_users.all_users.insert(user2.age, user2.clone());

    // println!("{:?}", all_users);
//     let x = 20;
//     let y= &x;
//     let z = & x;
//     let m = & z;
  
//   let numbers = vec![20, 30, 40];
// let mut iterable = numbers.iter();
//   println!("{}", iterable)

let student1  = Student{
  name: String::from("Bankat"),
  gender: Gender::Male,
};

getStudent(student1);
println!("{:?}", student1);
    }

  #[derive(Debug)]
  struct  Student{
      name: String,
      gender: Gender,
    }
#[derive(Debug)]
    pub enum Gender {
      Male,
      Female,
      Other
    }


fn  getStudent(Student: Student) {
  println!("{:?}", Student)
}
    
