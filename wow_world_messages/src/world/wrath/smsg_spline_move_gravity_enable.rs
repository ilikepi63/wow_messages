use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_spline_move_gravity_enable.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_spline_move_gravity_enable.wowm#L1):
/// ```text
/// smsg SMSG_SPLINE_MOVE_GRAVITY_ENABLE = 0x04D4 {
///     PackedGuid unit;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_GRAVITY_ENABLE {
    pub unit: Guid,
}

impl crate::Message for SMSG_SPLINE_MOVE_GRAVITY_ENABLE {
    const OPCODE: u32 = 0x04d4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(w);

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D4, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(r)?;

        Ok(Self {
            unit,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SPLINE_MOVE_GRAVITY_ENABLE {}

impl SMSG_SPLINE_MOVE_GRAVITY_ENABLE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
    }
}
