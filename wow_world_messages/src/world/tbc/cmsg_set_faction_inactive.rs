use std::convert::{TryFrom, TryInto};
use crate::tbc::Faction;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_inactive.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_inactive.wowm#L1):
/// ```text
/// cmsg CMSG_SET_FACTION_INACTIVE = 0x0317 {
///     Faction faction;
///     Bool inactive;
/// }
/// ```
pub struct CMSG_SET_FACTION_INACTIVE {
    pub faction: Faction,
    pub inactive: bool,
}

impl crate::Message for CMSG_SET_FACTION_INACTIVE {
    const OPCODE: u32 = 0x0317;

    fn size_without_header(&self) -> u32 {
        3
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // faction: Faction
        w.write_all(&(self.faction.as_int() as u16).to_le_bytes())?;

        // inactive: Bool
        w.write_all(u8::from(self.inactive).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 3 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0317, size: body_size as u32 });
        }

        // faction: Faction
        let faction: Faction = crate::util::read_u16_le(r)?.try_into()?;

        // inactive: Bool
        let inactive = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            faction,
            inactive,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_FACTION_INACTIVE {}
