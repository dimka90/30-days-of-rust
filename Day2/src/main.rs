
#[derive(Debug)]
struct User{
    username: String,
    age: u128,
    status: Status
}
impl User {

fn create_user(username: String, age: u128, status: Status) -> User{
      let new_user = User{
        username,
        age,
        status, 
    };
    new_user
}
fn go_off_line(user:&mut User){
    user.status = Status::Offline;
    }
fn go_on_line(user: &mut User){
user.status = Status::Online;
}
}
#[derive(Debug)]
enum Status{
    Online,
    Offline
}
fn main() {
   
    let mut user1 = User::create_user(String::from("DImka"), 19, Status::Online);
    println!("{} {} {:?}", user1.username, user1.age, user1.status);

    User::go_off_line(&mut user1);
    User::go_on_line(&mut user1);
    println!("{:?}",user1.status);
    println!("{:?}", user1);

}



