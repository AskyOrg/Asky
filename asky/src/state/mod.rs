use asky_text_component::prelude::{Component, MiniMessageError, parse_mini_message};
use base64::engine::general_purpose;
use base64::{Engine, alphabet, engine};
use std::io::Read;
use std::{fs::File, path::Path};
use thiserror::Error;

#[derive(Default)]
pub struct ServerState {
    motd: Component,
    time_world: i64,
    lock_time: bool,
    max_players: u32,
    show_online_player_count: bool,
    hardcore: bool,
    spawn_position: (f64, f64, f64),
    spawn_rotation: (f32, f32),
    view_distance: i32,
    fav_icon: Option<String>,
}

impl ServerState {
    pub const fn motd(&self) -> &Component {
        &self.motd
    }

    /// Start building a new `ServerState`.
    pub fn builder() -> ServerStateBuilder {
        ServerStateBuilder::default()
    }

    pub const fn time_world_ticks(&self) -> i64 {
        self.time_world
    }

    pub const fn is_time_locked(&self) -> bool {
        self.lock_time
    }

    pub const fn max_players(&self) -> u32 {
        self.max_players
    }

    pub const fn is_hardcore(&self) -> bool {
        self.hardcore
    }

    pub const fn spawn_position(&self) -> (f64, f64, f64) {
        self.spawn_position
    }

    pub const fn spawn_rotation(&self) -> (f32, f32) {
        self.spawn_rotation
    }

    pub const fn view_distance(&self) -> i32 {
        self.view_distance
    }

    pub fn fav_icon(&self) -> Option<String> {
        self.fav_icon.clone()
    }
}

#[derive(Default)]
pub struct ServerStateBuilder {
    description_text: String,
    time_world: i64,
    lock_time: bool,
    max_players: u32,
    show_online_player_count: bool,
    hardcore: bool,
    spawn_position: (f64, f64, f64),
    spawn_rotation: (f32, f32),
    view_distance: i32,
    fav_icon: Option<String>,
}

#[derive(Debug, Error)]
pub enum ServerStateBuilderError {
    #[error(transparent)]
    MiniMessage(#[from] MiniMessageError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl ServerStateBuilder {
    pub fn description_text<S>(&mut self, text: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.description_text = text.into();
        self
    }

    /// Set the time of the world
    pub const fn time_world(&mut self, time_world: i64) -> &mut Self {
        self.time_world = time_world;
        self
    }

    pub const fn lock_time(&mut self, lock_time: bool) -> &mut Self {
        self.lock_time = lock_time;
        self
    }

    pub const fn max_players(&mut self, max_players: u32) -> &mut Self {
        self.max_players = max_players;
        self
    }

    pub const fn show_online_player_count(&mut self, show: bool) -> &mut Self {
        self.show_online_player_count = show;
        self
    }

    pub const fn hardcore(&mut self, hardcore: bool) -> &mut Self {
        self.hardcore = hardcore;
        self
    }

    pub const fn spawn_position(&mut self, position: (f64, f64, f64)) -> &mut Self {
        self.spawn_position = position;
        self
    }

    pub const fn spawn_rotation(&mut self, rotation: (f32, f32)) -> &mut Self {
        self.spawn_rotation = rotation;
        self
    }

    pub fn view_distance(&mut self, view_distance: i32) -> &mut Self {
        self.view_distance = view_distance.max(0);
        self
    }

    pub fn fav_icon<P>(&mut self, file_path: P) -> Result<&mut Self, ServerStateBuilderError>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(file_path)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
        let base64_encoded = engine.encode(&buffer);

        self.fav_icon = Some(format!("data:image/png;base64,{base64_encoded}"));
        Ok(self)
    }

    /// Finish building
    pub fn build(self) -> Result<ServerState, ServerStateBuilderError> {
        Ok(ServerState {
            motd: parse_mini_message(&self.description_text)?,
            time_world: self.time_world,
            lock_time: self.lock_time,
            max_players: self.max_players,
            show_online_player_count: self.show_online_player_count,
            hardcore: self.hardcore,
            spawn_position: self.spawn_position,
            spawn_rotation: self.spawn_rotation,
            view_distance: self.view_distance,
            fav_icon: self.fav_icon,
        })
    }
}
