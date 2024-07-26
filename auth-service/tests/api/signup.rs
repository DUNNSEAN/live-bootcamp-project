use serde_json::json;

use crate::helpers::TestApp;

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