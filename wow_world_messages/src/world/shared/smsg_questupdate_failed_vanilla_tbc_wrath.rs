use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_failed.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_FAILED = 0x0196 {
///     u32 quest_id;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_FAILED {
    pub quest_id: u32,
}

impl crate::Message for SMSG_QUESTUPDATE_FAILED {
    const OPCODE: u32 = 0x0196;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0196, size: body_size as u32 });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            quest_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_QUESTUPDATE_FAILED {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_QUESTUPDATE_FAILED {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_QUESTUPDATE_FAILED {}
