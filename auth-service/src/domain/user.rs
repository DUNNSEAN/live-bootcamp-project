use super::{Email, Password};

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState, domain::{AuthAPIError, LoginAttemptId, TwoFACode}, routes::{LoginResponse, TwoFactorAuthResponse}, utils::auth::generate_auth_cookie
};

/// Represents a user in the authentication system with a particular state.
/// The state is represented by the generic type `T` which must implement the `UserStatus` trait.
#[derive(Clone, Debug, PartialEq)]
pub struct User<T: UserStatus> {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
    // This reassures the compiler that the parameter
    // gets used.
    marker: std::marker::PhantomData<T>,
}

impl User<UnverifiedUser> {
    /// Creates a new `User` in the `UnverifiedUser` state.
    ///
    /// # Arguments
    ///
    /// * `email` - The email of the user.
    /// * `password` - The password of the user.
    /// * `requires_2fa` - A boolean indicating if the user requires 2FA.
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
            marker: std::marker::PhantomData,
        }
    }

    /// Validates the user's credentials and transitions them to the `VerifiedUser` state if successful.
    ///
    /// # Arguments
    ///
    /// * `state` - The application state, used to access the user store for validation.
    ///
    /// # Returns
    ///
    /// * `Ok(User<VerifiedUser>)` - If the credentials are valid.
    /// * `Err(AuthAPIError::IncorrectCredentials)` - If the credentials are invalid.
    pub async fn validate(
        self,
        State(state): State<AppState>, 
    ) -> Result<User<VerifiedUser>, AuthAPIError> {
        let user_store = &state.user_store.read().await;
        if user_store.validate_user(&self.email, &self.password).await.is_err() {
            Err(AuthAPIError::IncorrectCredentials)
        } else {
            Ok(User {
                email: self.email,
                password: self.password,
                requires_2fa: self.requires_2fa,
                marker: std::marker::PhantomData,
            })
        }
    }
}

impl User<VerifiedUser> {
     /// Handles the logic for users requiring 2FA by sending a 2FA code and returning an appropriate response.
    ///
    /// # Arguments
    ///
    /// * `email` - The email of the verified user.
    /// * `state` - The application state, used to access the 2FA code store and email client.
    /// * `jar` - The cookie jar used to store cookies for the response.
    ///
    /// # Returns
    ///
    /// * `(CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>)` - The updated cookie jar and response or an error.
    pub async fn handle_2fa(
        email: &Email,
        state: &AppState,
        jar: CookieJar,
    ) -> (
        CookieJar,
        Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
    ) {
        let login_attempt_id = LoginAttemptId::default();
        let two_fa_code = TwoFACode::default();
    
        if state
            .two_fa_code_store
            .write()
            .await
            .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
            .await
            .is_err()
        {
            return (jar, Err(AuthAPIError::UnexpectedError));
        }
    
        if state
            .email_client
            .send_email(email, "2FA Code", two_fa_code.as_ref())
            .await
            .is_err()
        {
            return (jar, Err(AuthAPIError::UnexpectedError));
        }
    
        let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".to_owned(),
            login_attempt_id: login_attempt_id.as_ref().to_owned(),
        }));
    
        (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
    }

    /// Handles the logic for users not requiring 2FA by generating an authentication cookie and returning an appropriate response.
    ///
    /// # Arguments
    ///
    /// * `email` - The email of the verified user.
    /// * `jar` - The cookie jar used to store cookies for the response.
    ///
    /// # Returns
    ///
    /// * `(CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>)` - The updated cookie jar and response or an error.
    pub async fn handle_no_2fa(
        email: &Email,
        jar: CookieJar,
    ) -> (
        CookieJar,
        Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
    ) {
        let auth_cookie = match generate_auth_cookie(email) {
            Ok(cookie) => cookie,
            Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
        };
    
        let updated_jar = jar.add(auth_cookie);
    
        (
            updated_jar,
            Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))),
        )
    }
    
}

/// Represents a user that has not yet been verified (e.g., their credentials have not been validated).
pub enum UnverifiedUser {}

/// Represents a user that has been verified (e.g., their credentials have been validated).
pub enum VerifiedUser {}

/// A trait that all user status types must implement. Used to enforce typestate transitions.
pub trait UserStatus{}
impl UserStatus for UnverifiedUser {}
impl UserStatus for VerifiedUser {}