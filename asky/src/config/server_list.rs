use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct ServerListConfig {
    pub max_players: u32,
    pub motd: String,
    pub show_online_player_count: bool,
    pub server_icon: PathBuf,
}

impl Default for ServerListConfig {
    fn default() -> Self {
        Self {
            max_players: 20,
            motd: "\\u00a76$$\\u00a7d Asky Server\\u00a7r (\\u00a7bgithub.com/AskyOrg/Asky\\u00a7r) \\u00a76$$".into(),
            show_online_player_count: true,
            server_icon: PathBuf::from("server-icon.png"),
        }
    }
}
