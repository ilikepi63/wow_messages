use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_use_item.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_use_item.wowm#L23):
/// ```text
/// enum ClientCastFlags : u8 {
///     NONE = 0;
///     EXTRA = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum ClientCastFlags {
    None,
    Extra,
}

impl ClientCastFlags {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Extra => 0x2,
        }
    }

}

impl Default for ClientCastFlags {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for ClientCastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Extra => f.write_str("Extra"),
        }
    }
}

impl TryFrom<u8> for ClientCastFlags {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            2 => Ok(Self::Extra),
            v => Err(crate::errors::EnumError::new("ClientCastFlags", v as u32),)
        }
    }
}
