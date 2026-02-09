use crate::time::{TimeError, TimeState};

#[derive(Debug, Clone)]
pub struct CommandOutcome {
    pub success: bool,
    pub message: String,
    pub minutes_advanced: u32,
}

pub fn execute_command(state: &mut TimeState, command: &str) -> CommandOutcome {
    let normalized = normalize_command(command);
    if normalized.is_empty() {
        return CommandOutcome {
            success: false,
            message: "Empty command".to_string(),
            minutes_advanced: 0,
        };
    }

    if let Some(rest) = normalized.strip_prefix("tick ") {
        return match rest.parse::<u32>() {
            Ok(delta) => apply_delta(state, delta, format!("Ticked {} minute(s)", delta)),
            Err(_) => CommandOutcome {
                success: false,
                message: "Invalid tick value. Usage: tick <minutes>".to_string(),
                minutes_advanced: 0,
            },
        };
    }

    if normalized == "tick" {
        return CommandOutcome {
            success: false,
            message: "Usage: tick <minutes>".to_string(),
            minutes_advanced: 0,
        };
    }

    if let Some(cost) = state.action_costs().get(&normalized).copied() {
        return apply_delta(
            state,
            cost,
            format!("Executed '{}', advanced {} minute(s)", normalized, cost),
        );
    }

    CommandOutcome {
        success: false,
        message: format!("Unknown command: {}", normalized),
        minutes_advanced: 0,
    }
}

fn apply_delta(state: &mut TimeState, delta: u32, ok_message: String) -> CommandOutcome {
    match state.advance_minutes(delta) {
        Ok(()) => CommandOutcome {
            success: true,
            message: ok_message,
            minutes_advanced: delta,
        },
        Err(TimeError::DayOverflow) => CommandOutcome {
            success: false,
            message: "Failed to advance time: day overflow".to_string(),
            minutes_advanced: 0,
        },
    }
}

fn normalize_command(command: &str) -> String {
    command
        .split_whitespace()
        .map(|s| s.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tick() {
        let mut state = TimeState::default();
        let out = execute_command(&mut state, "tick 60");
        assert!(out.success);
        assert_eq!(out.minutes_advanced, 60);
        assert_eq!(state.timestamp().minute, 60);
    }

    #[test]
    fn parses_named_action_with_normalization() {
        let mut state = TimeState::default();
        let out = execute_command(&mut state, "  Craft   Totem ");
        assert!(out.success);
        assert_eq!(out.minutes_advanced, 40);
        assert_eq!(state.timestamp().minute, 40);
    }

    #[test]
    fn invalid_command_does_not_mutate_time() {
        let mut state = TimeState::default();
        let before = state.timestamp();

        let out = execute_command(&mut state, "unknown thing");

        assert!(!out.success);
        assert_eq!(out.minutes_advanced, 0);
        assert_eq!(state.timestamp(), before);
    }
}
