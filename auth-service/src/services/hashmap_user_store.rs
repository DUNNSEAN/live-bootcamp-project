use std::collections::HashMap;

use crate::domain::password::Password;
use crate::domain::user::User;
use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::email::Email;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }
    
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == *password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

// Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use crate::domain::{email, password};

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let email = email::Email::parse("test@email.com").unwrap();
        let password = password::Password::parse("passwordtest").unwrap();
        let user = User::new(email, password, false);

        // Test adding a user
        let result = user_store.add_user(user.clone()).await;
        assert_eq!(result, Ok(()));

        // Test adding a user with the same email (should fail)
        let result = user_store.add_user(user.clone()).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let email = email::Email::parse("test@email.com").unwrap();
        let password = password::Password::parse("passwordtest").unwrap();

        let user = User::new(email, password, false);

        // Test getting a user that doesn't exist
        let result = user_store.get_user(&user.email).await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));

        // Test getting a user that does exist
        user_store.add_user(user.clone()).await.unwrap();
        let result = user_store.get_user(&user.email).await;
        assert_eq!(result, Ok(user));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let email = email::Email::parse("test@email.com").unwrap();
        let password = password::Password::parse("passwordtest").unwrap();

        let user = User::new(email, password, false);

        // Test validating a user that doesn't exist
        let result = user_store.validate_user(&user.email, &user.password).await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));

        // Test validating a user with the wrong password
        user_store.add_user(user.clone()).await.unwrap();
        let bad_password = password::Password::parse("wrongpassword").unwrap();
        let result = user_store.validate_user(&user.email, &bad_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validating a user with the correct password
        let result = user_store.validate_user(&user.email, &user.password).await;
        assert_eq!(result, Ok(()));

        // Test validating a user with the correct password but with a different email
        // let bad_email = email::Email::parse("wrongemail").unwrap();  // Can't unwrap an invalid email
        // let result = user_store.validate_user(&bad_email, &user.password).await;
        // assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}