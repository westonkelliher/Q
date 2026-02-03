use mvp::game::{create_hardcoded_world, GameState};
use mvp::web::{create_router, SharedGameState};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Creating hardcoded MVP world...");
    let world = create_hardcoded_world();
    
    println!("Initializing game state...");
    let game_state = GameState::new(world);
    let shared_state: SharedGameState = Arc::new(std::sync::Mutex::new(game_state));
    
    let app = create_router(shared_state);
    
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");
    
    println!("Web server running at http://127.0.0.1:3000");
    println!("Open your browser and navigate to the URL above to play!");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
