use auth_service::{Application, services};
use auth_service::app_state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
#[tokio::main]
async fn main() {
    let user_store = services::hashmap_user_store::HashmapUserStore::default();
    let user_store = Arc::new(RwLock::new(user_store));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
