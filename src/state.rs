use core::default::Default;
use defmt::*;

#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Off,
    Low,
    Mid,
    High,
    Auto,
}

impl Format for State {
    fn format(&self, f: defmt::Formatter) {
        match self {
            State::Off => defmt::write!(f, "Off"),
            State::Auto => defmt::write!(f, "Auto"),
            State::Low => defmt::write!(f, "Low"),
            State::Mid => defmt::write!(f, "Mid"),
            State::High => defmt::write!(f, "High"),
        }
    }
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            0 => State::Off,
            1 => State::Low,
            2 => State::Mid,
            3 => State::High,
            4 => State::Auto,
            _ => State::Off,
        }
    }
}

impl From<State> for u8 {
    fn from(value: State) -> Self {
        match value {
            State::Off => 0,
            State::Low => 1,
            State::Mid => 2,
            State::High => 3,
            State::Auto => 4,
        }
    }
}

pub struct StateManager {
    pub state: State,
}

impl Default for StateManager {
    fn default() -> Self {
        StateManager::new()
    }
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            state: State::default(),
        }
    }

    pub fn next(&mut self) {
        let new_state = match self.state {
            State::Off => State::Low,
            State::Low => State::Mid,
            State::Mid => State::High,
            State::High => State::Auto,
            State::Auto => State::Low,
        };

        debug!("State transition: {:?} -> {:?}", self.state, new_state);
        self.state = new_state;
    }

    pub fn set_off(&mut self) {
        debug!("State transition: {:?} -> Off", self.state);
        self.state = State::Off;
    }
}
