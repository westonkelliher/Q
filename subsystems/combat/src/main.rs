use combat::{Combatant, CombatState, CombatResult};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    match args[1].as_str() {
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
            
            run_full_combat(health1, attack1, health2, attack2);
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
}

fn print_usage() {
    println!("Combat System CLI");
    println!();
    println!("Usage:");
    println!("  combat <health1> <attack1> <health2> <attack2>");
    println!("    Simulate full combat between two combatants");
    println!("    Example: combat 10 5 8 3");
    println!();
    println!("  combat-round <health1> <attack1> <health2> <attack2>");
    println!("    Execute one round of combat");
    println!("    Example: combat-round 10 5 8 3");
    println!();
    println!("Shorthands:");
    println!("  comb  - alias for combat");
    println!("  cr    - alias for combat-round");
}

fn run_full_combat(health1: i32, attack1: i32, health2: i32, attack2: i32) {
    let combatant1 = Combatant::new(health1, attack1);
    let combatant2 = Combatant::new(health2, attack2);
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
