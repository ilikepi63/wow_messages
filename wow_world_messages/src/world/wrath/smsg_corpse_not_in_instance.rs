use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_corpse_not_in_instance.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_corpse_not_in_instance.wowm#L1):
/// ```text
/// smsg SMSG_CORPSE_NOT_IN_INSTANCE = 0x0506 {
/// }
/// ```
pub struct SMSG_CORPSE_NOT_IN_INSTANCE {
}

impl crate::Message for SMSG_CORPSE_NOT_IN_INSTANCE {
    const OPCODE: u32 = 0x0506;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0506, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CORPSE_NOT_IN_INSTANCE {}
