use serde::Serialize;
use std::collections::HashMap;

pub const MINUTES_PER_DAY: u32 = 1440;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Lightzone {
    Morning,
    Afternoon,
    Night,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Timestamp {
    pub day: usize,
    pub minute: u16,
    pub lightzone: Lightzone,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClockAngles {
    pub hand_15_deg: f64,
    pub dial_45_deg_total: f64,
    pub dial_45_deg_visual: f64,
}

#[derive(Debug, Clone)]
pub struct TimeState {
    day: usize,
    minute: u16,
    action_costs: HashMap<String, u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeError {
    DayOverflow,
}

impl std::fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeError::DayOverflow => write!(f, "day overflow while advancing time"),
        }
    }
}

impl std::error::Error for TimeError {}

impl Default for TimeState {
    fn default() -> Self {
        let mut action_costs = HashMap::new();
        action_costs.insert("craft totem".to_string(), 40);

        Self {
            day: 0,
            minute: 0,
            action_costs,
        }
    }
}

impl TimeState {
    pub fn new(day: usize, minute: u16, action_costs: HashMap<String, u32>) -> Self {
        let normalized_minute = minute % (MINUTES_PER_DAY as u16);
        Self {
            day,
            minute: normalized_minute,
            action_costs,
        }
    }

    pub fn timestamp(&self) -> Timestamp {
        Timestamp {
            day: self.day,
            minute: self.minute,
            lightzone: lightzone_for_minute(self.minute),
        }
    }

    pub fn hhmm(&self) -> String {
        let hour = self.minute / 60;
        let minute = self.minute % 60;
        format!("{:02}:{:02}", hour, minute)
    }

    pub fn action_costs(&self) -> &HashMap<String, u32> {
        &self.action_costs
    }

    pub fn advance_minutes(&mut self, delta: u32) -> Result<(), TimeError> {
        let current = u32::from(self.minute);
        let total = current + delta;
        let day_carry = total / MINUTES_PER_DAY;
        let next_minute = (total % MINUTES_PER_DAY) as u16;

        let carry = usize::try_from(day_carry).map_err(|_| TimeError::DayOverflow)?;
        self.day = self.day.checked_add(carry).ok_or(TimeError::DayOverflow)?;
        self.minute = next_minute;
        Ok(())
    }

    pub fn clock_angles(&self) -> ClockAngles {
        let minute_of_day = f64::from(self.minute);
        let hand_15_deg = minute_of_day * 0.25;
        let dial_45_deg_total = minute_of_day * 0.75;
        let dial_45_deg_visual = dial_45_deg_total.rem_euclid(360.0);

        ClockAngles {
            hand_15_deg,
            dial_45_deg_total,
            dial_45_deg_visual,
        }
    }
}

pub fn lightzone_for_minute(minute: u16) -> Lightzone {
    match minute {
        240..=719 => Lightzone::Morning,
        720..=1199 => Lightzone::Afternoon,
        _ => Lightzone::Night,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_minutes_basic() {
        let mut state = TimeState::default();
        state.minute = 200;

        state.advance_minutes(40).unwrap();

        assert_eq!(state.timestamp().day, 0);
        assert_eq!(state.timestamp().minute, 240);
    }

    #[test]
    fn day_rollover_single() {
        let mut state = TimeState::default();
        state.minute = 1439;

        state.advance_minutes(1).unwrap();

        assert_eq!(state.timestamp().day, 1);
        assert_eq!(state.timestamp().minute, 0);
    }

    #[test]
    fn day_rollover_large_delta() {
        let mut state = TimeState::default();

        state.advance_minutes(3000).unwrap();

        assert_eq!(state.timestamp().day, 2);
        assert_eq!(state.timestamp().minute, 120);
    }

    #[test]
    fn lightzone_boundaries() {
        assert_eq!(lightzone_for_minute(239), Lightzone::Night);
        assert_eq!(lightzone_for_minute(240), Lightzone::Morning);
        assert_eq!(lightzone_for_minute(719), Lightzone::Morning);
        assert_eq!(lightzone_for_minute(720), Lightzone::Afternoon);
        assert_eq!(lightzone_for_minute(1199), Lightzone::Afternoon);
        assert_eq!(lightzone_for_minute(1200), Lightzone::Night);
    }

    #[test]
    fn overflow_guard() {
        let mut state = TimeState::default();
        state.day = usize::MAX;
        state.minute = 1439;

        let result = state.advance_minutes(1);

        assert_eq!(result, Err(TimeError::DayOverflow));
    }
}
