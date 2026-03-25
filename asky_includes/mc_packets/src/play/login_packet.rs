use crate::play::data::login_packet_data::post_v1_16::PostV1_16Data;
use crate::play::data::login_packet_data::post_v1_20_2::PostV1_20_2Data;
use crate::play::data::login_packet_data::pre_v1_16::{DimensionField, PreV1_16Data};
use mc_protocol::prelude::*;
use std::borrow::Cow;

#[derive(PacketOut)]
pub struct LoginPacket {
    /// The player's Entity ID (EID).
    entity_id: i32,
    data: LoginPacketData,
}

enum LoginPacketData {
    PreV1_16(PreV1_16Data),
    PostV1_16(PostV1_16Data),
    PostV1_20_2(PostV1_20_2Data),
}

impl EncodePacket for LoginPacketData {
    fn encode(
        &self,
        writer: &mut BinaryWriter,
        protocol_version: ProtocolVersion,
    ) -> Result<(), BinaryWriterError> {
        match self {
            LoginPacketData::PreV1_16(value) => value.encode(writer, protocol_version),
            LoginPacketData::PostV1_16(value) => value.encode(writer, protocol_version),
            LoginPacketData::PostV1_20_2(value) => value.encode(writer, protocol_version),
        }
    }
}

impl LoginPacket {
    /// This is the constructor for version 1.16.2 up to 1.18.2 included
    pub fn with_dimension_codec(
        dimension: Dimension,
        registry_codec_bytes: Cow<'static, [u8]>,
        dimension_codec_bytes: Cow<'static, [u8]>,
    ) -> Self {
        let iden = dimension.identifier();
        Self {
            entity_id: 0,
            data: LoginPacketData::PostV1_16(PostV1_16Data {
                dimension_names: LengthPaddedVec::new(vec![iden.clone()]),
                world_name: iden.clone(),
                v1_19_dimension_type: iden.clone(),
                registry_codec_bytes: Omitted::Some(registry_codec_bytes),
                v1_16_2_dimension_codec_bytes: Omitted::Some(dimension_codec_bytes),
                ..PostV1_16Data::default()
            }),
        }
    }

    /// This is the constructor for 1.16, 1.16.1 and 1.19 up to 1.20 included
    pub fn with_registry_codec(
        dimension: Dimension,
        registry_codec_bytes: Cow<'static, [u8]>,
    ) -> Self {
        let iden = dimension.identifier();
        Self {
            entity_id: 0,
            data: LoginPacketData::PostV1_16(PostV1_16Data {
                dimension_names: LengthPaddedVec::new(vec![iden.clone()]),
                world_name: iden.clone(),
                dimension_name: iden.clone(),
                registry_codec_bytes: Omitted::Some(registry_codec_bytes),
                v1_19_dimension_type: iden.clone(),
                ..PostV1_16Data::default()
            }),
        }
    }

    /// This is the constructor for all versions from 1.20.2 to 1.20.4 included
    pub fn with_dimension_post_v1_20_2(dimension: Dimension) -> Self {
        let iden = dimension.identifier();
        Self {
            entity_id: 0,
            data: LoginPacketData::PostV1_20_2(PostV1_20_2Data {
                dimension_names: LengthPaddedVec::new(vec![iden.clone()]),
                dimension_name: iden.clone(),
                dimension_type: iden.clone(),
                ..PostV1_20_2Data::default()
            }),
        }
    }

    /// This is the constructor for all versions from 1.7.2 to 1.15.2 included
    pub fn with_dimension_pre_v1_16(dimension: Dimension) -> Self {
        Self {
            entity_id: 0,
            data: LoginPacketData::PreV1_16(PreV1_16Data {
                dimension: DimensionField(dimension.legacy_i8()),
                ..PreV1_16Data::default()
            }),
        }
    }

    /// This is the constructor for all versions starting 1.20.5
    pub fn with_dimension_index(dimension: Dimension, dimension_index: i32) -> Self {
        let iden = dimension.identifier();
        Self {
            entity_id: 0,
            data: LoginPacketData::PostV1_20_2(PostV1_20_2Data {
                dimension_names: LengthPaddedVec::new(vec![iden.clone()]),
                dimension_name: iden.clone(),
                v1_20_5_dimension_type: dimension_index.into(),
                ..PostV1_20_2Data::default()
            }),
        }
    }

    pub fn set_game_mode(
        mut self,
        protocol_version: ProtocolVersion,
        game_mode: u8,
        is_hard_core: bool,
    ) -> Self {
        match &mut self.data {
            LoginPacketData::PreV1_16(value) => {
                if is_hard_core {
                    value.game_mode = game_mode | 0x8;
                } else {
                    value.game_mode = game_mode;
                }
            }
            LoginPacketData::PostV1_16(value) => {
                if is_hard_core && protocol_version.is_before_inclusive(ProtocolVersion::V1_16_1) {
                    value.game_mode = game_mode | 0x8;
                } else {
                    value.game_mode = game_mode;
                }
                value.v1_16_2_is_hardcore = is_hard_core;
            }
            LoginPacketData::PostV1_20_2(value) => {
                value.game_mode = game_mode;
                value.is_hardcore = is_hard_core;
            }
        }
        self
    }

    pub fn set_view_distance(mut self, view_distance: i32) -> Self {
        match &mut self.data {
            LoginPacketData::PreV1_16(value) => {
                value.v1_14_view_distance = view_distance.into();
            }
            LoginPacketData::PostV1_16(value) => {
                value.view_distance = view_distance.into();
                value.v1_18_simulation_distance = view_distance.into();
            }
            LoginPacketData::PostV1_20_2(value) => {
                value.view_distance = view_distance.into();
                value.simulation_distance = view_distance.into();
            }
        }
        self
    }

    pub fn set_reduced_debug_info(mut self, reduced_debug_info: bool) -> Self {
        match &mut self.data {
            LoginPacketData::PreV1_16(value) => {
                value.v1_8_reduced_debug_info = reduced_debug_info;
            }
            LoginPacketData::PostV1_16(value) => {
                value.reduced_debug_info = reduced_debug_info;
            }
            LoginPacketData::PostV1_20_2(value) => {
                value.reduced_debug_info = reduced_debug_info;
            }
        }
        self
    }
}
