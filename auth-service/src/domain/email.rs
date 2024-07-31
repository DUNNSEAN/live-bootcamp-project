#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, String> {
        if email.contains('@') {
            Ok(Email(email.to_string()))
        } else {
            Err("Invalid email address".to_string())
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::parse("test@example.com");
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::parse("invalid-email");
        assert!(email.is_err());
    }
}
