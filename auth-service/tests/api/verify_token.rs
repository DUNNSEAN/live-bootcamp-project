use serde_json::json;

use crate::helpers::TestApp;

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