use std::convert::{TryFrom, TryInto};
use crate::world::tbc::LfgData;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_lfg.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_lfg.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_LFG = 0x036E {
///     LfgData[3] data;
/// }
/// ```
pub struct SMSG_LFG_UPDATE_LFG {
    pub data: [LfgData; 3],
}

impl crate::Message for SMSG_LFG_UPDATE_LFG {
    const OPCODE: u32 = 0x036e;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // data: LfgData[3]
        for i in self.data.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036E, size: body_size as u32 });
        }

        // data: LfgData[3]
        let mut data = [LfgData::default(); 3];
        for i in data.iter_mut() {
            *i = LfgData::read(r)?;
        }

        Ok(Self {
            data,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_LFG_UPDATE_LFG {}
