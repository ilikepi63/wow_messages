use std::convert::{TryFrom, TryInto};
use crate::world::tbc::SpellCastTargets;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm#L1):
/// ```text
/// cmsg CMSG_CAST_SPELL = 0x012E {
///     u32 spell;
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_CAST_SPELL {
    pub spell: u32,
    pub targets: SpellCastTargets,
}

impl crate::Message for CMSG_CAST_SPELL {
    const OPCODE: u32 = 0x012e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=322).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x012E, size: body_size as u32 });
        }

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            spell,
            targets,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_CAST_SPELL {}

impl CMSG_CAST_SPELL {
    pub(crate) fn size(&self) -> usize {
        4 // spell: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}
