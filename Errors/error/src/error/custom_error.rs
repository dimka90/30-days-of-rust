#[derive(Debug)]
pub enum  AuthError{
    USERNOTFOUND,
    INCORRECTPASSWORD,
    INACTIVEUSER,
}