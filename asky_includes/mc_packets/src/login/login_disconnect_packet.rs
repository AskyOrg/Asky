use asky_text_component::prelude::Component;
use mc_protocol::prelude::*;

#[derive(PacketOut)]
pub struct LoginDisconnectPacket {
    /// Reason as a JSON Text Component
    pub reason: String,
}

impl Default for LoginDisconnectPacket {
    fn default() -> Self {
        Self {
            reason: r#"{"text":"Disconnected"}"#.to_owned(),
        }
    }
}

impl LoginDisconnectPacket {
    pub fn text(text: impl Into<String>) -> LoginDisconnectPacket {
        let component = Component::new(text);
        Self {
            reason: component.to_json(),
        }
    }
}
