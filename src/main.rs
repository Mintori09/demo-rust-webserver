#[tokio::main]
async fn main() {
    axum_auth_backend::run().await;
}
