use super::{email::Email, password::Password};

// The User struct should contain 3 fields. email, which is a String; 
// password, which is also a String; and requires_2fa, which is a boolean. 
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    requires_2fa: bool,
}

impl User {
    // Constructor function to create a new User instance
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}