use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie,
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar, // New!
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    //validation logic...
    let email = Email::parse(request.email);
    let password = Password::parse(request.password);

    let (email, password) = if let (Ok(email), Ok(password)) = (email, password) {
        (email, password)
    } else {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar, Ok(Err(AuthAPIError::IncorrectCredentials)));
    }

    // call `user_store.get_user`. Return AuthAPIError::IncorrectCredentials if the operation fails.
    if user_store.get_user(&email).await.is_err() {
        return (jar, Ok(Err(AuthAPIError::IncorrectCredentials)));
    }
    
    
    // Call the generate_auth_cookie function defined in the auth module.
    // If the function call fails return AuthAPIError::UnexpectedError.
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Ok(Err(AuthAPIError::UnexpectedError))),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(Ok(StatusCode::OK.into_response())))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
pub struct LoginResponse {
    pub message: String,
}