#[derive(Debug, PartialEq, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Self, &'static str> {
        if password.len() >= 8 {
            Ok(Password(password.to_string()))
        } else {
            Err("Password must be at least 8 characters long")
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let password = Password::parse("password123");
        assert!(password.is_ok());
    }

    #[test]
    fn test_invalid_password() {
        let password = Password::parse("123");
        assert!(password.is_err());
    }
}