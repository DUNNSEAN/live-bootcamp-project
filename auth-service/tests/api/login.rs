use crate::helpers::{get_random_email, TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME,ErrorResponse};

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.  
    let app = TestApp::new().await;   

    let body = serde_json::json!({
        "email": "sdunn@gmail.com",
        "password": "password",
        "requires2FA": false,
    });

    app.post_signup(&body).await;
    
    let test_case = serde_json::json!({
        "email": "incorrect@email.com",
        "password": "incorrectpassword"
    });

    let response = app.post_login(&test_case).await;

    assert_eq!(
        response.status().as_u16(),
        401,
        "Failed for input: {:?}",
        test_case
    );

    let body: ErrorResponse = response.json().await.unwrap();
    assert_eq!(
        body.error,
        "Incorrect credentials",
        "Failed for input: {:?}",
        test_case
    );


}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message. 
    let app = TestApp::new().await;
    
    let test_case = serde_json::json!({
        "email": "invalid_email",
        "password": "invpass"
    });

    let response = app.post_login(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        400,
        "Failed for input: {:?}",
        test_case
    );

    let body: ErrorResponse = response.json().await.unwrap();
    assert_eq!(
        body.error,
        "Invalid credentials",
        "Failed for input: {:?}",
        test_case
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    
    let test_case = serde_json::json!({
        "email": "sdunn@gmail.com",
    });

    let response = app.post_login(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        422,
        "Failed for input: {:?}",
        test_case
    );
}