use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_event_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_event_invite.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_INVITE = 0x0431 {
///     Guid event;
///     Guid invite_id;
///     CString name;
///     Bool pre_event;
///     Bool guild_event;
/// }
/// ```
pub struct CMSG_CALENDAR_EVENT_INVITE {
    pub event: Guid,
    pub invite_id: Guid,
    pub name: String,
    pub pre_event: bool,
    pub guild_event: bool,
}

impl crate::Message for CMSG_CALENDAR_EVENT_INVITE {
    const OPCODE: u32 = 0x0431;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // pre_event: Bool
        w.write_all(u8::from(self.pre_event).to_le_bytes().as_slice())?;

        // guild_event: Bool
        w.write_all(u8::from(self.guild_event).to_le_bytes().as_slice())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(19..=274).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0431, size: body_size as u32 });
        }

        // event: Guid
        let event = Guid::read(r)?;

        // invite_id: Guid
        let invite_id = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // pre_event: Bool
        let pre_event = crate::util::read_u8_le(r)? != 0;
        // guild_event: Bool
        let guild_event = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            event,
            invite_id,
            name,
            pre_event,
            guild_event,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CALENDAR_EVENT_INVITE {}

impl CMSG_CALENDAR_EVENT_INVITE {
    pub(crate) fn size(&self) -> usize {
        8 // event: Guid
        + 8 // invite_id: Guid
        + self.name.len() + 1 // name: CString
        + 1 // pre_event: Bool
        + 1 // guild_event: Bool
    }
}
