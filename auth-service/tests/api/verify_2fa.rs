use serde_json::json;

use crate::helpers::TestApp;

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

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}