use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_totem_destroyed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_totem_destroyed.wowm#L1):
/// ```text
/// cmsg CMSG_TOTEM_DESTROYED = 0x0413 {
///     u8 slot;
/// }
/// ```
pub struct CMSG_TOTEM_DESTROYED {
    pub slot: u8,
}

impl crate::Message for CMSG_TOTEM_DESTROYED {
    const OPCODE: u32 = 0x0413;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0413, size: body_size as u32 });
        }

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            slot,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_TOTEM_DESTROYED {}
