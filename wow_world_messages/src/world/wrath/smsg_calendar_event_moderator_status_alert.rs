use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_event_moderator_status_alert.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_event_moderator_status_alert.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT = 0x0445 {
///     PackedGuid invitee;
///     Guid event_id;
///     u8 rank;
///     Bool show_alert;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {
    pub invitee: Guid,
    pub event_id: Guid,
    pub rank: u8,
    pub show_alert: bool,
}

impl crate::Message for SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {
    const OPCODE: u32 = 0x0445;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // invitee: PackedGuid
        self.invitee.write_packed_guid_into_vec(w);

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // rank: u8
        w.write_all(&self.rank.to_le_bytes())?;

        // show_alert: Bool
        w.write_all(u8::from(self.show_alert).to_le_bytes().as_slice())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(12..=19).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0445, size: body_size as u32 });
        }

        // invitee: PackedGuid
        let invitee = Guid::read_packed(r)?;

        // event_id: Guid
        let event_id = Guid::read(r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(r)?;

        // show_alert: Bool
        let show_alert = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            invitee,
            event_id,
            rank,
            show_alert,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {}

impl SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {
    pub(crate) fn size(&self) -> usize {
        self.invitee.size() // invitee: Guid
        + 8 // event_id: Guid
        + 1 // rank: u8
        + 1 // show_alert: Bool
    }
}
