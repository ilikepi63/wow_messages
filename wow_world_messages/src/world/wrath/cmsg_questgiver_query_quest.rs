use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_questgiver_query_quest.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_questgiver_query_quest.wowm#L8):
/// ```text
/// cmsg CMSG_QUESTGIVER_QUERY_QUEST = 0x0186 {
///     Guid guid;
///     u32 quest_id;
///     u8 unknown1;
/// }
/// ```
pub struct CMSG_QUESTGIVER_QUERY_QUEST {
    pub guid: Guid,
    pub quest_id: u32,
    pub unknown1: u8,
}

impl crate::Message for CMSG_QUESTGIVER_QUERY_QUEST {
    const OPCODE: u32 = 0x0186;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0186, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            quest_id,
            unknown1,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_QUESTGIVER_QUERY_QUEST {}
