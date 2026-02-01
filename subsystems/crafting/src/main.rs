use crafting::{Registry, cli};

fn main() {
    // Initialize registry with sample content
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    // Print welcome message if running interactively
    if atty::is(atty::Stream::Stdin) {
        eprintln!("Crafting System CLI");
        eprintln!("Type 'help' for available commands");
        eprintln!();
    }
    
    // Run REPL
    if let Err(e) = cli::run_repl(&mut registry) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
