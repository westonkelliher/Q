use crafting::{Registry, cli};

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let human_readable = args.iter().any(|arg| arg == "--human-readable" || arg == "-h");
    
    // Initialize registry with sample content
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    // Print welcome message if running interactively
    if atty::is(atty::Stream::Stdin) {
        eprintln!("Crafting System CLI");
        eprintln!("Type 'help' for available commands");
        if human_readable {
            eprintln!("Output format: Human-readable");
        } else {
            eprintln!("Output format: JSON (use --human-readable for readable format)");
        }
        eprintln!();
    }
    
    // Run REPL
    if let Err(e) = cli::run_repl(&mut registry, human_readable) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
