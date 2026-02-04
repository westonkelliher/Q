use crate::game::{create_hardcoded_world, execute_command, GameState};
use std::io::{self, BufRead, Write};

/// Run interactive REPL mode
pub fn run_repl() {
    let world = create_hardcoded_world();
    let mut state = GameState::new(world);
    
    println!("MVP CLI REPL - Type 'help' for commands, 'quit' to exit");
    
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    loop {
        print!("> ");
        stdout.flush().unwrap();
        
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        
        let command = input.trim().to_lowercase();
        
        if command == "quit" || command == "q" {
            println!("Goodbye!");
            break;
        }
        
        let (success, message) = execute_command(&mut state, &command);
        if !success {
            println!("Error: {}", message);
        } else {
            println!("{}", message);
        }
    }
}

/// Run script execution mode
pub fn run_script(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let world = create_hardcoded_world();
    let mut state = GameState::new(world);
    
    println!("Executing script: {}", path);
    
    let file = std::fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let command = line.trim();
        
        // Skip empty lines and comments
        if command.is_empty() || command.starts_with('#') {
            continue;
        }
        
        println!("[{}] > {}", line_num + 1, command);
        let (success, message) = execute_command(&mut state, command);
        if !success {
            println!("Error: {}", message);
        } else {
            println!("{}", message);
        }
    }
    
    println!("\nScript execution complete.");
    Ok(())
}
