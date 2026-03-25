use mc_protocol::prelude::{ProtocolVersion, State};

#[derive(PartialEq, Eq)]
pub enum KeepAliveStatus {
    Disabled,
    ShouldEnable,
    Enabled,
}

pub struct ClientState {
    state: State,
    protocol_version: ProtocolVersion,
    kick_message: Option<String>,
    message_id: i32,
    keep_alive_enabled: KeepAliveStatus,
    feet_y: f64,
    is_flight_allowed: bool,
    is_flying: bool,
    flying_speed: f32,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            state: State::Handshake,
            protocol_version: ProtocolVersion::Any,
            kick_message: None,
            message_id: -1,
            keep_alive_enabled: KeepAliveStatus::Disabled,
            feet_y: 0.0,
            is_flight_allowed: false,
            is_flying: false,
            flying_speed: 0.05,
        }
    }
}

impl ClientState {
    const ANONYMOUS: &'static str = "Anonymous";

    // Kick

    pub fn kick(&mut self, kick_message: &str) {
        self.kick_message = Some(kick_message.to_string());
    }

    pub fn should_kick(&self) -> Option<String> {
        self.kick_message.clone()
    }

    // State

    pub const fn state(&self) -> State {
        self.state
    }

    pub const fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    // Protocol version

    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    pub const fn set_protocol_version(&mut self, new_protocol_version: ProtocolVersion) {
        self.protocol_version = new_protocol_version;
    }

    // Keep alive

    pub fn should_enable_keep_alive(&self) -> bool {
        self.keep_alive_enabled == KeepAliveStatus::ShouldEnable
    }

    pub fn set_keep_alive_should_enable(&mut self) {
        if self.keep_alive_enabled == KeepAliveStatus::Disabled {
            self.keep_alive_enabled = KeepAliveStatus::ShouldEnable;
        }
    }

    pub fn set_keep_alive_enabled(&mut self) {
        if self.keep_alive_enabled == KeepAliveStatus::ShouldEnable {
            self.keep_alive_enabled = KeepAliveStatus::Enabled;
        }
    }

    // Position

    pub const fn get_y_position(&self) -> f64 {
        self.feet_y
    }

    pub const fn set_feet_position(&mut self, feet_y: f64) {
        self.feet_y = feet_y;
    }

    // Movement

    pub const fn is_flight_allowed(&self) -> bool {
        self.is_flight_allowed
    }

    pub const fn set_is_flight_allowed(&mut self, allow_flight: bool) {
        self.is_flight_allowed = allow_flight;
    }

    pub const fn is_flying(&self) -> bool {
        self.is_flying
    }

    pub const fn set_is_flying(&mut self, is_flying: bool) {
        self.is_flying = is_flying;
    }

    pub const fn get_flying_speed(&self) -> f32 {
        self.flying_speed
    }

    pub const fn set_flying_speed(&mut self, flying_speed: f32) {
        self.flying_speed = flying_speed;
    }
}
