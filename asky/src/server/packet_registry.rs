use crate::server::batch::Batch;
use crate::server::client_state::ClientState;
use crate::server::packet_handler::{PacketHandler, PacketHandlerError};
use crate::state::ServerState;
use macros::PacketReport;
use mc_packets::config::acknowledge_finish_configuration_packet::AcknowledgeConfigurationPacket;
use mc_packets::config::client_bound_known_packs_packet::ClientBoundKnownPacksPacket;
use mc_packets::config::configuration_client_bound_plugin_message_packet::ConfigurationClientBoundPluginMessagePacket;
use mc_packets::config::finish_configuration_packet::FinishConfigurationPacket;
use mc_packets::config::registry_data_packet::RegistryDataPacket;
use mc_packets::config::update_tags_packet::UpdateTagsPacket;
use mc_packets::handshaking::handshake_packet::HandshakePacket;
use mc_packets::login::custom_query_answer_packet::CustomQueryAnswerPacket;
use mc_packets::login::custom_query_packet::CustomQueryPacket;
use mc_packets::login::game_profile_packet::GameProfilePacket;
use mc_packets::login::login_acknowledged_packet::LoginAcknowledgedPacket;
use mc_packets::login::login_disconnect_packet::LoginDisconnectPacket;
use mc_packets::login::login_state_packet::LoginStartPacket;
use mc_packets::login::login_success_packet::LoginSuccessPacket;
use mc_packets::login::set_compression_packet::SetCompressionPacket;
use mc_packets::play::login_packet::LoginPacket;
use mc_packets::status::ping_request_packet::PingRequestPacket;
use mc_packets::status::ping_response_packet::PongResponsePacket;
use mc_packets::status::status_request_packet::StatusRequestPacket;
use mc_packets::status::status_response_packet::StatusResponsePacket;
use mc_protocol::prelude::*;
use net::raw_packet::RawPacket;

#[derive(PacketReport)]
pub enum PacketRegistry {
    // Handshake packets
    #[protocol_id(
        state = "handshake",
        bound = "serverbound",
        name = "minecraft:intention"
    )]
    Handshake(HandshakePacket),

    // Status packets
    #[protocol_id(
        state = "status",
        bound = "serverbound",
        name = "minecraft:status_request"
    )]
    StatusRequest(StatusRequestPacket),

    #[protocol_id(
        state = "status",
        bound = "clientbound",
        name = "minecraft:status_response"
    )]
    StatusResponse(StatusResponsePacket),

    #[protocol_id(
        state = "status",
        bound = "serverbound",
        name = "minecraft:ping_request"
    )]
    PingRequest(PingRequestPacket),

    #[protocol_id(
        state = "status",
        bound = "clientbound",
        name = "minecraft:pong_response"
    )]
    PongResponse(PongResponsePacket),

    // Login packets
    #[protocol_id(state = "login", bound = "serverbound", name = "minecraft:hello")]
    LoginStart(LoginStartPacket),

    #[protocol_id(
        state = "login",
        bound = "serverbound",
        name = "minecraft:login_acknowledged"
    )]
    LoginAcknowledged(LoginAcknowledgedPacket),

    #[protocol_id(
        state = "login",
        bound = "serverbound",
        name = "minecraft:custom_query_answer"
    )]
    CustomQueryAnswer(CustomQueryAnswerPacket),

    #[protocol_id(
        state = "login",
        bound = "clientbound",
        name = "minecraft:custom_query"
    )]
    CustomQuery(CustomQueryPacket),

    #[protocol_id(
        state = "login",
        bound = "clientbound",
        name = "minecraft:login_finished"
    )]
    LoginSuccess(LoginSuccessPacket),

    #[protocol_id(
        state = "login",
        bound = "clientbound",
        name = "minecraft:game_profile"
    )]
    GameProfile(GameProfilePacket),

    #[protocol_id(
        state = "login",
        bound = "clientbound",
        name = "minecraft:login_disconnect"
    )]
    LoginDisconnect(LoginDisconnectPacket),

    #[protocol_id(
        state = "login",
        bound = "clientbound",
        name = "minecraft:login_compression"
    )]
    SetCompression(SetCompressionPacket),

    // Configuration packets
    #[protocol_id(
        state = "configuration",
        bound = "serverbound",
        name = "minecraft:finish_configuration"
    )]
    AcknowledgeConfiguration(AcknowledgeConfigurationPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        name = "minecraft:custom_payload"
    )]
    ConfigurationClientBoundPluginMessage(ConfigurationClientBoundPluginMessagePacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        name = "minecraft:select_known_packs"
    )]
    ClientBoundKnownPacks(ClientBoundKnownPacksPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        name = "minecraft:registry_data"
    )]
    RegistryData(RegistryDataPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        name = "minecraft:update_tags"
    )]
    UpdateTags(UpdateTagsPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        name = "minecraft:finish_configuration"
    )]
    FinishConfiguration(FinishConfigurationPacket),

    #[protocol_id(
        state = "configuration",
        bound = "clientbound",
        name = "minecraft:disconnect"
    )]
    ConfigurationDisconnect(DisconnectPacket),

    // Play packets
    #[protocol_id(state = "play", bound = "clientbound", name = "minecraft:login")]
    Login(Box<LoginPacket>),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        name = "minecraft:player_position"
    )]
    SynchronizePlayerPosition(SynchronizePlayerPositionPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        name = "minecraft:move_player_pos"
    )]
    SetPlayerPosition(SetPlayerPositionPacket),

    #[protocol_id(
        state = "play",
        bound = "serverbound",
        name = "minecraft:move_player_pos_rot"
    )]
    SetPlayerPositionAndRotation(SetPlayerPositionAndRotationPacket),

    #[protocol_id(
        state = "play",
        bound = "clientbound",
        name = "minecraft:set_default_spawn_position"
    )]
    SetDefaultSpawnPosition(SetDefaultSpawnPositionPacket),

    #[protocol_id(state = "play", bound = "clientbound", name = "minecraft:keep_alive")]
    ClientBoundKeepAlive(ClientBoundKeepAlivePacket),

    #[protocol_id(state = "play", bound = "clientbound", name = "minecraft:disconnect")]
    PlayDisconnect(DisconnectPacket),

    #[protocol_id(state = "play", bound = "clientbound", name = "minecraft:set_time")]
    UpdateTime(UpdateTimePacket),
}

impl PacketHandler for PacketRegistry {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch<PacketRegistry>, PacketHandlerError> {
        match self {
            Self::Handshake(packet) => packet.handle(client_state, server_state),
            Self::StatusRequest(packet) => packet.handle(client_state, server_state),
            Self::PingRequest(packet) => packet.handle(client_state, server_state),
            Self::LoginStart(packet) => packet.handle(client_state, server_state),
            Self::CustomQueryAnswer(packet) => packet.handle(client_state, server_state),
            Self::LoginAcknowledged(packet) => packet.handle(client_state, server_state),
            Self::AcknowledgeConfiguration(packet) => packet.handle(client_state, server_state),
            Self::SetPlayerPositionAndRotation(packet) => packet.handle(client_state, server_state),
            Self::SetPlayerPosition(packet) => packet.handle(client_state, server_state),
            _ => Err(PacketHandlerError::custom("Unhandled packet")),
        }
    }
}
