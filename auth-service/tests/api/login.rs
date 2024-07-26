use serde_json::json;

use crate::helpers::TestApp;

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