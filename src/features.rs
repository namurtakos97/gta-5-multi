use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GameFeatures {
    pub aimbot: bool,
    pub speed_hack: bool,
    pub god_mode: bool,
}

impl GameFeatures {
    pub fn new() -> Self {
        GameFeatures {
            aimbot: false,
            speed_hack: false,
            god_mode: false,
        }
    }

    pub fn toggle_aimbot(&mut self) {
        self.aimbot = !self.aimbot;
    }

    pub fn toggle_speed_hack(&mut self) {
        self.speed_hack = !self.speed_hack;
    }

    pub fn toggle_god_mode(&mut self) {
        self.god_mode = !self.god_mode;
    }
}