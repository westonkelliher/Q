use clap::{Parser, Subcommand};
use mvp::game::create_hardcoded_world;
use mvp::web::{create_router, SharedGameState};
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(name = "mvp")]
#[command(about = "MVP Game - Web or CLI mode")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run interactive CLI REPL
    Cli,
    /// Execute commands from a script file
    Script { 
        /// Path to the script file
        file: String 
    },
    /// Run web server (default)
    Web,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Cli) => {
            mvp::cli::run_repl();
        }
        Some(Commands::Script { file }) => {
            if let Err(e) = mvp::cli::run_script(&file) {
                eprintln!("Error running script: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Web) | None => {
            run_web_server().await;
        }
    }
}

async fn run_web_server() {
    println!("Creating crafting registry...");
    let mut crafting_registry = mvp::game::CraftingRegistry::new();
    mvp::game::crafting::content::register_sample_content(&mut crafting_registry);
    
    println!("Creating hardcoded MVP world...");
    let world = create_hardcoded_world(&mut crafting_registry);
    
    println!("Initializing game state...");
    let game_state = mvp::game::GameState::new(world, crafting_registry);
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
