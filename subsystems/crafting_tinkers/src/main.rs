use crafting::{Registry, cli};
use colored::*;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let json_mode = args.iter().any(|arg| arg == "--json" || arg == "-j");
    let human_readable = !json_mode; // Default to human-readable unless --json is specified
    
    // Initialize registry with sample content
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    // Print welcome message if running interactively
    if atty::is(atty::Stream::Stdin) {
        eprintln!("╔═══════════════════════════════════════════════════════════╗");
        eprintln!("║         Crafting & Combat System CLI                      ║");
        eprintln!("╚═══════════════════════════════════════════════════════════╝");
        eprintln!();
        eprintln!("Quick Start Examples:");
        eprintln!("  {} - List all items", "list items".cyan());
        eprintln!("  {} - List all recipes", "list recipes".cyan());
        eprintln!("  {} - Create a new item", "new copper_ore".cyan());
        eprintln!("  {} - Show inventory", "inventory".cyan());
        eprintln!("  {} - Simulate combat", "combat 10 5 8 3".cyan());
        eprintln!("  {} - Execute one combat round", "combat-round 10 5 8 3".cyan());
        eprintln!("  {} - Show help", "help".cyan());
        eprintln!();
        if human_readable {
            eprintln!("Output format: {} (use {} for JSON)", 
                "Human-readable".green(), "--json".yellow());
        } else {
            eprintln!("Output format: {} (default is human-readable)", 
                "JSON".yellow());
        }
        eprintln!();
    }
    
    // Run REPL
    if let Err(e) = cli::run_repl(&mut registry, human_readable) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
