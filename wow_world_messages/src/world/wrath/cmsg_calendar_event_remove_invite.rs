use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_event_remove_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_event_remove_invite.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_REMOVE_INVITE = 0x0433 {
///     Guid event;
///     Guid sender_invite_id;
///     Guid invite_id;
/// }
/// ```
pub struct CMSG_CALENDAR_EVENT_REMOVE_INVITE {
    pub event: Guid,
    pub sender_invite_id: Guid,
    pub invite_id: Guid,
}

impl crate::Message for CMSG_CALENDAR_EVENT_REMOVE_INVITE {
    const OPCODE: u32 = 0x0433;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // sender_invite_id: Guid
        w.write_all(&self.sender_invite_id.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0433, size: body_size as u32 });
        }

        // event: Guid
        let event = Guid::read(r)?;

        // sender_invite_id: Guid
        let sender_invite_id = Guid::read(r)?;

        // invite_id: Guid
        let invite_id = Guid::read(r)?;

        Ok(Self {
            event,
            sender_invite_id,
            invite_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CALENDAR_EVENT_REMOVE_INVITE {}
