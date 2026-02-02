use combat::{Combatant, CombatState, CombatResult};
use std::io::{self, Write};

struct CombatSession {
    side1: Option<Combatant>,
    side2: Option<Combatant>,
}

impl CombatSession {
    fn new() -> Self {
        Self {
            side1: None,
            side2: None,
        }
    }

    fn set_side1(&mut self, health: i32, attack: i32) {
        self.side1 = Some(Combatant::new(health, attack));
        println!("Side 1 set: HP={}, ATK={}", health, attack);
    }

    fn set_side2(&mut self, health: i32, attack: i32) {
        self.side2 = Some(Combatant::new(health, attack));
        println!("Side 2 set: HP={}, ATK={}", health, attack);
    }

    fn show(&self) {
        println!("Current Combat State:");
        match self.side1 {
            Some(c) => println!("  Side 1: HP={}, ATK={}", c.health, c.attack),
            None => println!("  Side 1: Not set"),
        }
        match self.side2 {
            Some(c) => println!("  Side 2: HP={}, ATK={}", c.health, c.attack),
            None => println!("  Side 2: Not set"),
        }
    }

    fn fight(&self) -> Result<(), String> {
        let combatant1 = self.side1.ok_or_else(|| "Side 1 not set. Use 'set-side1 <health> <attack>' first.".to_string())?;
        let combatant2 = self.side2.ok_or_else(|| "Side 2 not set. Use 'set-side2 <health> <attack>' first.".to_string())?;
        
        run_full_combat(combatant1, combatant2);
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // If arguments provided, run in one-shot mode (backward compatibility)
    if args.len() > 1 {
        match args[1].as_str() {
            "interactive" | "i" | "repl" => {
                run_interactive();
            }
            "combat" | "comb" => {
                if args.len() < 6 {
                    eprintln!("Error: combat requires 4 arguments: <health1> <attack1> <health2> <attack2>");
                    eprintln!("Example: combat 10 5 8 3");
                    std::process::exit(1);
                }
                
                let health1: i32 = args[2].parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid health1: {}", args[2]);
                    std::process::exit(1);
                });
                let attack1: i32 = args[3].parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid attack1: {}", args[3]);
                    std::process::exit(1);
                });
                let health2: i32 = args[4].parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid health2: {}", args[4]);
                    std::process::exit(1);
                });
                let attack2: i32 = args[5].parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid attack2: {}", args[5]);
                    std::process::exit(1);
                });
                
                run_full_combat(Combatant::new(health1, attack1), Combatant::new(health2, attack2));
            }
            "combat-round" | "cr" => {
                if args.len() < 6 {
                    eprintln!("Error: combat-round requires 4 arguments: <health1> <attack1> <health2> <attack2>");
                    eprintln!("Example: combat-round 10 5 8 3");
                    std::process::exit(1);
                }
                
                let health1: i32 = args[2].parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid health1: {}", args[2]);
                    std::process::exit(1);
                });
                let attack1: i32 = args[3].parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid attack1: {}", args[3]);
                    std::process::exit(1);
                });
                let health2: i32 = args[4].parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid health2: {}", args[4]);
                    std::process::exit(1);
                });
                let attack2: i32 = args[5].parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid attack2: {}", args[5]);
                    std::process::exit(1);
                });
                
                run_single_round(health1, attack1, health2, attack2);
            }
            "help" | "-h" | "--help" => {
                print_usage();
            }
            _ => {
                eprintln!("Error: Unknown command: {}", args[1]);
                print_usage();
                std::process::exit(1);
            }
        }
    } else {
        // No arguments - start interactive mode
        run_interactive();
    }
}

fn run_interactive() {
    let mut session = CombatSession::new();
    
    println!("Combat System - Interactive Mode");
    println!("Type 'help' for commands, 'quit' or 'exit' to exit");
    println!();
    
    loop {
        print!("combat> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                
                match parts[0] {
                    "quit" | "exit" | "q" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" | "h" => {
                        print_interactive_help();
                    }
                    "set-side1" | "side1" => {
                        if parts.len() < 3 {
                            eprintln!("Error: set-side1 requires 2 arguments: <health> <attack>");
                            eprintln!("Example: set-side1 10 5");
                            continue;
                        }
                        
                        match (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                            (Ok(health), Ok(attack)) => {
                                session.set_side1(health, attack);
                            }
                            (Err(_), _) => {
                                eprintln!("Error: Invalid health: {}", parts[1]);
                            }
                            (_, Err(_)) => {
                                eprintln!("Error: Invalid attack: {}", parts[2]);
                            }
                        }
                    }
                    "set-side2" | "side2" => {
                        if parts.len() < 3 {
                            eprintln!("Error: set-side2 requires 2 arguments: <health> <attack>");
                            eprintln!("Example: set-side2 8 3");
                            continue;
                        }
                        
                        match (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                            (Ok(health), Ok(attack)) => {
                                session.set_side2(health, attack);
                            }
                            (Err(_), _) => {
                                eprintln!("Error: Invalid health: {}", parts[1]);
                            }
                            (_, Err(_)) => {
                                eprintln!("Error: Invalid attack: {}", parts[2]);
                            }
                        }
                    }
                    "show" | "status" | "s" => {
                        session.show();
                    }
                    "fight" | "go" | "rip" | "f" => {
                        match session.fight() {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            }
                        }
                    }
                    _ => {
                        eprintln!("Unknown command: {}. Type 'help' for available commands.", parts[0]);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn print_usage() {
    println!("Combat System CLI");
    println!();
    println!("Interactive Mode (default):");
    println!("  Run without arguments to start interactive mode");
    println!("  Or use: combat interactive");
    println!();
    println!("One-shot Commands:");
    println!("  combat <health1> <attack1> <health2> <attack2>");
    println!("    Simulate full combat between two combatants");
    println!("    Example: combat 10 5 8 3");
    println!("    Alias: comb");
    println!();
    println!("  combat-round <health1> <attack1> <health2> <attack2>");
    println!("    Execute one round of combat");
    println!("    Example: combat-round 10 5 8 3");
    println!("    Alias: cr");
}

fn print_interactive_help() {
    println!("Available Commands:");
    println!("  set-side1 <health> <attack>  Set combatant 1 stats (alias: side1)");
    println!("  set-side2 <health> <attack>  Set combatant 2 stats (alias: side2)");
    println!("  show                         Display current combatant states (alias: status, s)");
    println!("  fight                         Run combat with saved combatants (aliases: go, rip, f)");
    println!("  help                          Show this help message (alias: h)");
    println!("  quit                          Exit the program (aliases: exit, q)");
    println!();
    println!("Examples:");
    println!("  combat> set-side1 10 5");
    println!("  combat> set-side2 8 3");
    println!("  combat> show");
    println!("  combat> fight");
}

fn run_full_combat(combatant1: Combatant, combatant2: Combatant) {
    let state = CombatState::new(combatant1, combatant2);
    
    let mut history = Vec::new();
    let mut current_state = state.clone();
    
    loop {
        let round_before = current_state.round;
        let health1_before = current_state.combatant1.health;
        let health2_before = current_state.combatant2.health;
        
        let result = current_state.execute_round();
        
        history.push((round_before + 1, health1_before, health2_before, 
                     current_state.combatant1.health, current_state.combatant2.health, result));
        
        match result {
            CombatResult::Ongoing => continue,
            _ => {
                println!("Combat Result: {}", format_result(result));
                println!("Total Rounds: {}", current_state.round);
                println!("Combatant 1: HP={}, ATK={}", 
                    current_state.combatant1.health, 
                    current_state.combatant1.attack);
                println!("Combatant 2: HP={}, ATK={}", 
                    current_state.combatant2.health, 
                    current_state.combatant2.attack);
                
                if !history.is_empty() {
                    println!("\nRound History:");
                    for (round, c1_before, c2_before, c1_after, c2_after, _) in history {
                        println!("  Round {}: C1 {} -> {}, C2 {} -> {}",
                            round, c1_before, c1_after, c2_before, c2_after);
                    }
                }
                break;
            }
        }
    }
}

fn run_single_round(health1: i32, attack1: i32, health2: i32, attack2: i32) {
    let combatant1 = Combatant::new(health1, attack1);
    let combatant2 = Combatant::new(health2, attack2);
    let mut state = CombatState::new(combatant1, combatant2);
    
    let health1_before = state.combatant1.health;
    let health2_before = state.combatant2.health;
    
    let result = state.execute_round();
    
    println!("Round: {}", state.round);
    println!("Combatant 1: {} -> {} (ATK={})", 
        health1_before, state.combatant1.health, state.combatant1.attack);
    println!("Combatant 2: {} -> {} (ATK={})", 
        health2_before, state.combatant2.health, state.combatant2.attack);
    println!("Result: {}", format_result(result));
}

fn format_result(result: CombatResult) -> &'static str {
    match result {
        CombatResult::Combatant1Wins => "Combatant 1 Wins",
        CombatResult::Combatant2Wins => "Combatant 2 Wins",
        CombatResult::Draw => "Draw",
        CombatResult::Ongoing => "Ongoing",
    }
}
