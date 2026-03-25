use serde::{Deserialize, Serialize};

use crate::config::world::time::TimeConfig;

pub mod time;

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct WorldConfig {
    /// Position to spawn the players at
    pub spawn_position: (f64, f64, f64),

    /// Rotation to spawn the players at
    pub spawn_rotation: (f32, f32),

    /// Time of the world
    /// Supported: "sunrise", "noon", "sunset", "midnight" or ticks (0 - 24000)
    pub time: TimeConfig,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            spawn_position: (0.0, 320.0, 0.0),
            spawn_rotation: (0.0, 0.0),
            time: TimeConfig::default(),
        }
    }
}
