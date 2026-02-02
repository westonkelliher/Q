use combat::{Combatant, CombatState, CombatResult};
use std::io::{self, Write};

struct CombatSession {
    side1: Vec<Combatant>,
    side2: Vec<Combatant>,
}

impl CombatSession {
    fn new() -> Self {
        Self {
            side1: Vec::new(),
            side2: Vec::new(),
        }
    }

    fn add_side1(&mut self, health: i32, attack: i32, leadership: i32) -> Result<(), String> {
        let combatant = Combatant::new(health, attack, leadership);
        
        // If this is the first combatant, it becomes the leader
        if self.side1.is_empty() {
            self.side1.push(combatant);
            println!("Side 1 Leader added: HP={}, ATK={}, Leadership={}", health, attack, leadership);
            return Ok(());
        }
        
        // Check if we can add a follower
        let leader = &self.side1[0];
        let max_size = (leader.leadership + 1) as usize;
        if self.side1.len() >= max_size {
            return Err(format!("Cannot add follower: team size {} would exceed leader's leadership capacity of {}", self.side1.len() + 1, max_size));
        }
        
        self.side1.push(combatant);
        println!("Added to Side 1: HP={}, ATK={}, Leadership={} (Position: {})", health, attack, leadership, self.side1.len());
        Ok(())
    }

    fn add_side2(&mut self, health: i32, attack: i32, leadership: i32) -> Result<(), String> {
        let combatant = Combatant::new(health, attack, leadership);
        
        // If this is the first combatant, it becomes the leader
        if self.side2.is_empty() {
            self.side2.push(combatant);
            println!("Side 2 Leader added: HP={}, ATK={}, Leadership={}", health, attack, leadership);
            return Ok(());
        }
        
        // Check if we can add a follower
        let leader = &self.side2[0];
        let max_size = (leader.leadership + 1) as usize;
        if self.side2.len() >= max_size {
            return Err(format!("Cannot add follower: team size {} would exceed leader's leadership capacity of {}", self.side2.len() + 1, max_size));
        }
        
        self.side2.push(combatant);
        println!("Added to Side 2: HP={}, ATK={}, Leadership={} (Position: {})", health, attack, leadership, self.side2.len());
        Ok(())
    }

    fn clear_side1(&mut self) {
        self.side1.clear();
        println!("Side 1 cleared");
    }

    fn clear_side2(&mut self) {
        self.side2.clear();
        println!("Side 2 cleared");
    }

    fn remove_side1(&mut self) -> Result<(), String> {
        if self.side1.is_empty() {
            return Err("Side 1 is already empty".to_string());
        }
        let removed = self.side1.pop().unwrap();
        println!("Removed from Side 1: HP={}, ATK={}, Leadership={}", removed.health, removed.attack, removed.leadership);
        Ok(())
    }

    fn remove_side2(&mut self) -> Result<(), String> {
        if self.side2.is_empty() {
            return Err("Side 2 is already empty".to_string());
        }
        let removed = self.side2.pop().unwrap();
        println!("Removed from Side 2: HP={}, ATK={}, Leadership={}", removed.health, removed.attack, removed.leadership);
        Ok(())
    }

    fn show(&self) {
        println!("Current Combat State:");
        if self.side1.is_empty() {
            println!("  Side 1: No combatants");
        } else {
            print!("  Side 1: ");
            for (i, c) in self.side1.iter().enumerate() {
                let role = if i == 0 { "Leader" } else { &format!("Follower {}", i) };
                print!("[{}: HP={}, ATK={}, LDR={}] ", role, c.health, c.attack, c.leadership);
            }
            println!();
        }
        
        if self.side2.is_empty() {
            println!("  Side 2: No combatants");
        } else {
            print!("  Side 2: ");
            for (i, c) in self.side2.iter().enumerate() {
                let role = if i == 0 { "Leader" } else { &format!("Follower {}", i) };
                print!("[{}: HP={}, ATK={}, LDR={}] ", role, c.health, c.attack, c.leadership);
            }
            println!();
        }
    }

    fn fight(&self) -> Result<(), String> {
        if self.side1.is_empty() {
            return Err("Side 1 has no combatants. Use 'add-side1 <health> <attack> <leadership>' first.".to_string());
        }
        if self.side2.is_empty() {
            return Err("Side 2 has no combatants. Use 'add-side2 <health> <attack> <leadership>' first.".to_string());
        }
        
        let state = CombatState::new(self.side1.clone(), self.side2.clone())
            .map_err(|e| format!("Invalid team configuration: {}", e))?;
        
        run_full_combat(state);
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
                // Parse format: combat <health1> <attack1> <leadership1> [health2 attack2 leadership2 ...] -- <health1> <attack1> <leadership1> [...]
                let separator_pos = args.iter().position(|a| a == "--");
                if separator_pos.is_none() {
                    eprintln!("Error: combat requires '--' separator between sides");
                    eprintln!("Format: combat <health1> <attack1> <leadership1> [...] -- <health1> <attack1> <leadership1> [...]");
                    eprintln!("Example: combat 10 5 3 -- 8 3 3");
                    eprintln!("Example: combat 10 5 3 8 3 2 -- 12 4 3");
                    std::process::exit(1);
                }
                
                let sep_pos = separator_pos.unwrap();
                if sep_pos < 2 || sep_pos >= args.len() - 1 {
                    eprintln!("Error: Invalid argument format");
                    std::process::exit(1);
                }
                
                let side1_args = &args[2..sep_pos];
                let side2_args = &args[sep_pos + 1..];
                
                if side1_args.len() % 3 != 0 {
                    eprintln!("Error: Side 1 arguments must be in groups of 3 (health attack leadership)");
                    std::process::exit(1);
                }
                if side2_args.len() % 3 != 0 {
                    eprintln!("Error: Side 2 arguments must be in groups of 3 (health attack leadership)");
                    std::process::exit(1);
                }
                
                let mut side1 = Vec::new();
                for chunk in side1_args.chunks(3) {
                    let health: i32 = chunk[0].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid health: {}", chunk[0]);
                        std::process::exit(1);
                    });
                    let attack: i32 = chunk[1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid attack: {}", chunk[1]);
                        std::process::exit(1);
                    });
                    let leadership: i32 = chunk[2].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid leadership: {}", chunk[2]);
                        std::process::exit(1);
                    });
                    side1.push(Combatant::new(health, attack, leadership));
                }
                
                let mut side2 = Vec::new();
                for chunk in side2_args.chunks(3) {
                    let health: i32 = chunk[0].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid health: {}", chunk[0]);
                        std::process::exit(1);
                    });
                    let attack: i32 = chunk[1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid attack: {}", chunk[1]);
                        std::process::exit(1);
                    });
                    let leadership: i32 = chunk[2].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid leadership: {}", chunk[2]);
                        std::process::exit(1);
                    });
                    side2.push(Combatant::new(health, attack, leadership));
                }
                
                match CombatState::new(side1, side2) {
                    Ok(state) => run_full_combat(state),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            "combat-round" | "cr" => {
                // Parse format: combat-round <health1> <attack1> <leadership1> [health2 attack2 leadership2 ...] -- <health1> <attack1> <leadership1> [...]
                let separator_pos = args.iter().position(|a| a == "--");
                if separator_pos.is_none() {
                    eprintln!("Error: combat-round requires '--' separator between sides");
                    eprintln!("Format: combat-round <health1> <attack1> <leadership1> [...] -- <health1> <attack1> <leadership1> [...]");
                    eprintln!("Example: combat-round 10 5 3 -- 8 3 3");
                    std::process::exit(1);
                }
                
                let sep_pos = separator_pos.unwrap();
                if sep_pos < 2 || sep_pos >= args.len() - 1 {
                    eprintln!("Error: Invalid argument format");
                    std::process::exit(1);
                }
                
                let side1_args = &args[2..sep_pos];
                let side2_args = &args[sep_pos + 1..];
                
                if side1_args.len() % 3 != 0 {
                    eprintln!("Error: Side 1 arguments must be in groups of 3 (health attack leadership)");
                    std::process::exit(1);
                }
                if side2_args.len() % 3 != 0 {
                    eprintln!("Error: Side 2 arguments must be in groups of 3 (health attack leadership)");
                    std::process::exit(1);
                }
                
                let mut side1 = Vec::new();
                for chunk in side1_args.chunks(3) {
                    let health: i32 = chunk[0].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid health: {}", chunk[0]);
                        std::process::exit(1);
                    });
                    let attack: i32 = chunk[1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid attack: {}", chunk[1]);
                        std::process::exit(1);
                    });
                    let leadership: i32 = chunk[2].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid leadership: {}", chunk[2]);
                        std::process::exit(1);
                    });
                    side1.push(Combatant::new(health, attack, leadership));
                }
                
                let mut side2 = Vec::new();
                for chunk in side2_args.chunks(3) {
                    let health: i32 = chunk[0].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid health: {}", chunk[0]);
                        std::process::exit(1);
                    });
                    let attack: i32 = chunk[1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid attack: {}", chunk[1]);
                        std::process::exit(1);
                    });
                    let leadership: i32 = chunk[2].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid leadership: {}", chunk[2]);
                        std::process::exit(1);
                    });
                    side2.push(Combatant::new(health, attack, leadership));
                }
                
                match CombatState::new(side1, side2) {
                    Ok(mut state) => run_single_round(&mut state),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
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
                    "add-side1" | "add1" => {
                        if parts.len() < 4 {
                            eprintln!("Error: add-side1 requires 3 arguments: <health> <attack> <leadership>");
                            eprintln!("Example: add-side1 10 5 3");
                            continue;
                        }
                        
                        match (parts[1].parse::<i32>(), parts[2].parse::<i32>(), parts[3].parse::<i32>()) {
                            (Ok(health), Ok(attack), Ok(leadership)) => {
                                if let Err(e) = session.add_side1(health, attack, leadership) {
                                    eprintln!("Error: {}", e);
                                }
                            }
                            (Err(_), _, _) => {
                                eprintln!("Error: Invalid health: {}", parts[1]);
                            }
                            (_, Err(_), _) => {
                                eprintln!("Error: Invalid attack: {}", parts[2]);
                            }
                            (_, _, Err(_)) => {
                                eprintln!("Error: Invalid leadership: {}", parts[3]);
                            }
                        }
                    }
                    "add-side2" | "add2" => {
                        if parts.len() < 4 {
                            eprintln!("Error: add-side2 requires 3 arguments: <health> <attack> <leadership>");
                            eprintln!("Example: add-side2 8 3 3");
                            continue;
                        }
                        
                        match (parts[1].parse::<i32>(), parts[2].parse::<i32>(), parts[3].parse::<i32>()) {
                            (Ok(health), Ok(attack), Ok(leadership)) => {
                                if let Err(e) = session.add_side2(health, attack, leadership) {
                                    eprintln!("Error: {}", e);
                                }
                            }
                            (Err(_), _, _) => {
                                eprintln!("Error: Invalid health: {}", parts[1]);
                            }
                            (_, Err(_), _) => {
                                eprintln!("Error: Invalid attack: {}", parts[2]);
                            }
                            (_, _, Err(_)) => {
                                eprintln!("Error: Invalid leadership: {}", parts[3]);
                            }
                        }
                    }
                    "clear-side1" | "clear1" => {
                        session.clear_side1();
                    }
                    "clear-side2" | "clear2" => {
                        session.clear_side2();
                    }
                    "remove-side1" | "remove1" => {
                        if let Err(e) = session.remove_side1() {
                            eprintln!("Error: {}", e);
                        }
                    }
                    "remove-side2" | "remove2" => {
                        if let Err(e) = session.remove_side2() {
                            eprintln!("Error: {}", e);
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
    println!("  combat <health1> <attack1> <leadership1> [...] -- <health1> <attack1> <leadership1> [...]");
    println!("    Simulate full combat between teams");
    println!("    Example: combat 10 5 3 -- 8 3 3");
    println!("    Example: combat 10 5 3 8 3 2 -- 12 4 3");
    println!("    Alias: comb");
    println!();
    println!("  combat-round <health1> <attack1> <leadership1> [...] -- <health1> <attack1> <leadership1> [...]");
    println!("    Execute one round of combat");
    println!("    Example: combat-round 10 5 3 -- 8 3 3");
    println!("    Alias: cr");
}

fn print_interactive_help() {
    println!("Available Commands:");
    println!("  add-side1 <health> <attack> <leadership>  Add combatant to side 1 (alias: add1)");
    println!("  add-side2 <health> <attack> <leadership>  Add combatant to side 2 (alias: add2)");
    println!("  clear-side1                              Clear all combatants from side 1 (alias: clear1)");
    println!("  clear-side2                              Clear all combatants from side 2 (alias: clear2)");
    println!("  remove-side1                             Remove last combatant from side 1 (alias: remove1)");
    println!("  remove-side2                             Remove last combatant from side 2 (alias: remove2)");
    println!("  show                                     Display current team states (alias: status, s)");
    println!("  fight                                     Run combat with saved teams (aliases: go, rip, f)");
    println!("  help                                      Show this help message (alias: h)");
    println!("  quit                                      Exit the program (aliases: exit, q)");
    println!();
    println!("Examples:");
    println!("  combat> add-side1 10 5 3");
    println!("  combat> add-side1 8 3 2");
    println!("  combat> add-side2 12 4 3");
    println!("  combat> show");
    println!("  combat> fight");
}

fn run_full_combat(mut state: CombatState) {
    let mut history = Vec::new();
    
    loop {
        let round_before = state.round;
        let side1_before: Vec<(i32, i32)> = state.side1.iter().map(|c| (c.health, c.attack)).collect();
        let side2_before: Vec<(i32, i32)> = state.side2.iter().map(|c| (c.health, c.attack)).collect();
        
        let result = state.execute_round();
        
        let side1_after: Vec<(i32, i32)> = state.side1.iter().map(|c| (c.health, c.attack)).collect();
        let side2_after: Vec<(i32, i32)> = state.side2.iter().map(|c| (c.health, c.attack)).collect();
        
        history.push((round_before + 1, side1_before, side2_before, side1_after, side2_after, result));
        
        match result {
            CombatResult::Ongoing => continue,
            _ => {
                println!("Combat Result: {}", format_result(result));
                println!("Total Rounds: {}", state.round);
                
                println!("\nFinal State:");
                println!("  Side 1: {} combatant(s)", state.side1.len());
                for (i, c) in state.side1.iter().enumerate() {
                    let role = if i == 0 { "Leader" } else { &format!("Follower {}", i) };
                    println!("    {}: HP={}, ATK={}, LDR={}", role, c.health, c.attack, c.leadership);
                }
                println!("  Side 2: {} combatant(s)", state.side2.len());
                for (i, c) in state.side2.iter().enumerate() {
                    let role = if i == 0 { "Leader" } else { &format!("Follower {}", i) };
                    println!("    {}: HP={}, ATK={}, LDR={}", role, c.health, c.attack, c.leadership);
                }
                
                if !history.is_empty() {
                    println!("\nRound History:");
                    for (round, s1_before, s2_before, s1_after, s2_after, _) in history {
                        println!("  Round {}:", round);
                        print!("    Side 1: ");
                        for (i, (h_before, a_before)) in s1_before.iter().enumerate() {
                            if i < s1_after.len() {
                                print!("[{} -> {} (ATK={})] ", h_before, s1_after[i].0, a_before);
                            } else {
                                print!("[{} (defeated)] ", h_before);
                            }
                        }
                        println!();
                        print!("    Side 2: ");
                        for (i, (h_before, a_before)) in s2_before.iter().enumerate() {
                            if i < s2_after.len() {
                                print!("[{} -> {} (ATK={})] ", h_before, s2_after[i].0, a_before);
                            } else {
                                print!("[{} (defeated)] ", h_before);
                            }
                        }
                        println!();
                    }
                }
                break;
            }
        }
    }
}

fn run_single_round(state: &mut CombatState) {
    let side1_before: Vec<(i32, i32)> = state.side1.iter().map(|c| (c.health, c.attack)).collect();
    let side2_before: Vec<(i32, i32)> = state.side2.iter().map(|c| (c.health, c.attack)).collect();
    
    let result = state.execute_round();
    
    println!("Round: {}", state.round);
    println!("Side 1:");
    for (i, (h_before, a_before)) in side1_before.iter().enumerate() {
        if i < state.side1.len() {
            let role = if i == 0 { "Leader" } else { &format!("Follower {}", i) };
            println!("  {}: {} -> {} (ATK={})", role, h_before, state.side1[i].health, a_before);
        } else {
            let role = if i == 0 { "Leader" } else { &format!("Follower {}", i) };
            println!("  {}: {} -> defeated (ATK={})", role, h_before, a_before);
        }
    }
    println!("Side 2:");
    for (i, (h_before, a_before)) in side2_before.iter().enumerate() {
        if i < state.side2.len() {
            let role = if i == 0 { "Leader" } else { &format!("Follower {}", i) };
            println!("  {}: {} -> {} (ATK={})", role, h_before, state.side2[i].health, a_before);
        } else {
            let role = if i == 0 { "Leader" } else { &format!("Follower {}", i) };
            println!("  {}: {} -> defeated (ATK={})", role, h_before, a_before);
        }
    }
    println!("Result: {}", format_result(result));
}

fn format_result(result: CombatResult) -> &'static str {
    match result {
        CombatResult::Side1Wins => "Side 1 Wins",
        CombatResult::Side2Wins => "Side 2 Wins",
        CombatResult::Draw => "Draw",
        CombatResult::Ongoing => "Ongoing",
    }
}
