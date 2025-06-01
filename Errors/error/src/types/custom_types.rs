use std::string;

use crate::error::custom_error::AuthError;

#[derive(Debug, Clone)]
pub struct User{
pub username: String,
pub password: String,
pub is_active: bool,
}
#[derive(Debug)]
pub  struct Users{
    pub users: Vec<User>
}

impl  Users {
    pub fn add_new_user(&mut self, user: User) -> &User{
        self.users.push(user);
        &self.users.last().unwrap()
    }

    fn find_user<'a>(&'a self, username: &'a str) -> Option<&'a User>{

    for user in &self.users{

        if  user.username == username{

                  return Some(user)
        }
    };

    return  None
    }



    pub fn authenticate<'a>(&'a self, username: &'a str, password: &'a str) -> Result<&'a User, AuthError> {
       
        for user  in &self.users{

            if user.username == username && user.password == password{
                    
                    return  Ok(user);
        
            }
            else if user.username == username && user.password != password{
                 
                return  Err(AuthError::INCORRECTPASSWORD)
            }

        };

        Err(AuthError::USERNOTFOUND)
}

    
}
impl  User {


    pub fn new( username: String, password: String) -> Self{
        Self { username,
            password, 
            is_active:
            true 
        }
    }
}