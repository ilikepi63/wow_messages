use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_clear_extra_aura_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_clear_extra_aura_info.wowm#L1):
/// ```text
/// smsg SMSG_CLEAR_EXTRA_AURA_INFO = 0x03A6 {
///     PackedGuid unit;
///     u32 spell;
/// }
/// ```
pub struct SMSG_CLEAR_EXTRA_AURA_INFO {
    pub unit: Guid,
    pub spell: u32,
}

impl crate::Message for SMSG_CLEAR_EXTRA_AURA_INFO {
    const OPCODE: u32 = 0x03a6;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(w);

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03A6, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        Ok(Self {
            unit,
            spell,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_CLEAR_EXTRA_AURA_INFO {}

impl SMSG_CLEAR_EXTRA_AURA_INFO {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
        + 4 // spell: u32
    }
}
