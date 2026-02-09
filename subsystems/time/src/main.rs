use std::sync::{Arc, Mutex};
use time_subsystem::{create_router, TimeState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let shared = Arc::new(Mutex::new(TimeState::default()));
    let app = create_router(shared);

    let listener = TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("failed to bind on 127.0.0.1:3001");

    println!("Time subsystem running at http://127.0.0.1:3001");

    axum::serve(listener, app).await.expect("server crashed");
}
