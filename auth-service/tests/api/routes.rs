use serde_json::json;

use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_returns_200() {
    let app = TestApp::new().await;
    let email = "test@email.com";
    let password = "test_password";
    let required_2fa = false;

    let body = json!({
        "email": email,
        "password": password,
        "required_2fa": required_2fa,
    });

    let response = app.signup(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_returns_200() {
    let app = TestApp::new().await;
    let email = "test@email.com";
    let password = "test_password";

    let body = json!({
        "email": email,
        "password": password,
    });

    let response = app.login(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_returns_200() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_returns_200() {
    let app = TestApp::new().await;
    let email = "test_email";
    let login_attempt_id = 1;
    let two_factor_code = 123456;

    let body = json!({
        "email": email,
        "login_attempt_id": login_attempt_id,
        "2fa_code": two_factor_code,
    });

    let response = app.verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_returns_200() {
    let app = TestApp::new().await;
    let token = "test_token";

    let body = json!({
        "token": token,
    });

    let response = app.verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
