use protocol_version_macro::Pvn;
use std::cmp::PartialEq;

/// Use french docs: https://fr.minecraft.wiki/w/Version_de_protocole
/// You can see also: https://minecraft.wiki/w/Java_Edition_protocol/Packets

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Pvn)]
#[repr(i32)]
pub enum ProtocolVersion {
    /// V26_1 released yesterday (I think)
    /// V26_6 = 775 ?? (I think)

    /// V 1.21.x
    #[default]
    #[pvn(packets = V1_21_9)]
    V1_21_11 = 774,
    #[pvn(data = V1_21_6)]
    V1_21_9 = 773,
    #[pvn(packets = V1_21_6, data = V1_21_6)]
    V1_21_7 = 772,
    V1_21_6 = 771,
    V1_21_5 = 770,
    V1_21_4 = 769,
    V1_21_2 = 768,
    V1_21 = 767,

    /// V 1.20.x
    V1_20_5 = 766,
    #[pvn(data = V1_20_2)]
    V1_20_3 = 765,
    V1_20_2 = 764,
    V1_20 = 763,

    /// V 1.19.x
    V1_19_4 = 762,
    #[pvn(data = V1_19)]
    V1_19_3 = 761,
    #[pvn(data = V1_19)]
    V1_19_1 = 760,
    V1_19 = 759,

    /// V 1.18.x
    #[pvn(packets = V1_18)]
    V1_18_2 = 758,
    V1_18 = 757,

    /// V 1.17.x
    #[pvn(packets = V1_17, data = V1_17)]
    V1_17_1 = 756,
    V1_17 = 755,

    /// V 1.16.x
    #[pvn(packets = V1_16_2, data = V1_16_2)]
    V1_16_4 = 754,
    #[pvn(packets = V1_16_2, data = V1_16_2)]
    V1_16_3 = 753,
    V1_16_2 = 751,
    #[pvn(packets = V1_16, data = V1_16)]
    V1_16_1 = 736,
    V1_16 = 735,

    /// V 1.15.x
    #[pvn(packets = V1_15)]
    V1_15_2 = 578,
    #[pvn(packets = V1_15)]
    V1_15_1 = 575,
    V1_15 = 573,

    /// V 1.14.x
    #[pvn(packets = V1_14)]
    V1_14_4 = 498,
    #[pvn(packets = V1_14)]
    V1_14_3 = 490,
    #[pvn(packets = V1_14)]
    V1_14_2 = 485,
    #[pvn(packets = V1_14)]
    V1_14_1 = 480,
    V1_14 = 477,

    /// V 1.13.x
    #[pvn(packets = V1_13)]
    V1_13_2 = 404,
    #[pvn(packets = V1_13)]
    V1_13_1 = 401,
    V1_13 = 393,

    /// V 1.12.x
    #[pvn(packets = V1_12_1)]
    V1_12_2 = 340,
    V1_12_1 = 338,
    V1_12 = 335,

    /// V 1.11.x
    #[pvn(packets = V1_11)]
    V1_11_1 = 316,
    V1_11 = 315,

    /// V 1.10.x
    V1_10 = 210,

    /// V 1.9.x
    V1_9_3 = 110,
    #[pvn(packets = V1_9)]
    V1_9_2 = 109,
    #[pvn(packets = V1_9)]
    V1_9_1 = 108,
    V1_9 = 107,

    /// V 1.8.x
    V1_8 = 47,

    /// V 1.7.x
    #[pvn(packets = V1_7_2)]
    V1_7_6 = 5,
    V1_7_2 = 4,

    /// A special value to represent any protocol version.
    Any = -1,

    /// A special value to represent an unknown protocol version.
    #[pvn(packets = Any)]
    Unsupported = -2,
}

impl ProtocolVersion {
    #[inline]
    pub fn between_inclusive(&self, min_version: Self, max_version: Self) -> bool {
        self >= &min_version && self <= &max_version
    }

    #[inline]
    pub fn is_after_inclusive(&self, other: Self) -> bool {
        self >= &other
    }

    #[inline]
    pub fn is_before_inclusive(&self, max_version: Self) -> bool {
        self <= &max_version
    }

    #[inline]
    pub fn supports_configuration_state(&self) -> bool {
        self.is_after_inclusive(ProtocolVersion::V1_20_2)
    }

    #[inline]
    pub fn is_modern(&self) -> bool {
        self.is_after_inclusive(ProtocolVersion::V1_13)
    }

    #[inline]
    pub fn is_any(&self) -> bool {
        *self == Self::Any
    }

    #[inline]
    pub fn is_unsupported(&self) -> bool {
        *self == Self::Unsupported
    }
}
