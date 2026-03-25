use mc_protocol::prelude::*;
use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketLengthParseError {
    #[error("packet_in length cannot be negative")]
    NegativeLength,
    #[error("packet_in length is too large")]
    PacketTooLarge,
    #[error(transparent)]
    BinaryReader(#[from] BinaryReaderError),
    #[error("var int is too long")]
    VarIntTooLong,
    #[error(transparent)]
    TryFromInt(#[from] TryFromIntError),
}

pub const MAXIMUM_PACKET_LENGTH: usize = 2_097_151;

pub fn get_packet_length(bytes: &[u8]) -> Result<usize, PacketLengthParseError> {
    let mut reader = BinaryReader::new(bytes);
    let packet_length = reader.read::<VarInt>()?.inner();

    if packet_length >= 0 {
        let packet_length = usize::try_from(packet_length)?;

        if packet_length > MAXIMUM_PACKET_LENGTH {
            Err(PacketLengthParseError::PacketTooLarge)
        } else {
            Ok(packet_length)
        }
    } else {
        Err(PacketLengthParseError::NegativeLength)
    }
}
