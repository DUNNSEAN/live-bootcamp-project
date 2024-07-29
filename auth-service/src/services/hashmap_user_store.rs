use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user),
            None => Err(UserStoreError::UserNotFound),
        }
    }
    
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new("test@email.com".to_string(), "passwordtest".to_string(), false);

        // Test adding a user
        let result = user_store.add_user(user.clone());
        assert_eq!(result, Ok(()));

        // Test adding a user with the same email (should fail)
        let result = user_store.add_user(user.clone());
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new("test@email.com".to_string(), "passwordtest".to_string(), false);

        // Test getting a user that doesn't exist
        let result = user_store.get_user(&user.email);
        assert_eq!(result, Err(UserStoreError::UserNotFound));

        // Test getting a user that does exist
        user_store.add_user(user.clone()).unwrap();
        let result = user_store.get_user(&user.email);
        assert_eq!(result, Ok(&user));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new("test@email.com".to_string(), "passwordtest".to_string(), false);

        // Test validating a user that doesn't exist
        let result = user_store.validate_user(&user.email, &user.password);
        assert_eq!(result, Err(UserStoreError::UserNotFound));

        // Test validating a user with the wrong password
        user_store.add_user(user.clone()).unwrap();
        let result = user_store.validate_user(&user.email, "wrongpassword");
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validating a user with the correct password
        let result = user_store.validate_user(&user.email, &user.password);
        assert_eq!(result, Ok(()));

        // Test validating a user with the correct password but with a different email
        let result = user_store.validate_user("wrongemail", &user.password);
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}