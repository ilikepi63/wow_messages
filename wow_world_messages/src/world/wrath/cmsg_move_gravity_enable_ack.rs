use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_move_gravity_enable_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_move_gravity_enable_ack.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_GRAVITY_ENABLE_ACK = 0x04D1 {
///     PackedGuid guid;
///     u32 unknown;
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_GRAVITY_ENABLE_ACK {
    pub guid: Guid,
    pub unknown: u32,
    pub info: MovementInfo,
}

impl crate::Message for CMSG_MOVE_GRAVITY_ENABLE_ACK {
    const OPCODE: u32 = 0x04d1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(36..=97).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D1, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            guid,
            unknown,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_MOVE_GRAVITY_ENABLE_ACK {}

impl CMSG_MOVE_GRAVITY_ENABLE_ACK {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // unknown: u32
        + self.info.size() // info: MovementInfo
    }
}
