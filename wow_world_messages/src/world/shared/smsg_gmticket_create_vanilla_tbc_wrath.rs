use std::convert::{TryFrom, TryInto};
use crate::world::shared::gm_ticket_response_vanilla_tbc_wrath::GmTicketResponse;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_create.wowm#L3):
/// ```text
/// smsg SMSG_GMTICKET_CREATE = 0x0206 {
///     GmTicketResponse response;
/// }
/// ```
pub struct SMSG_GMTICKET_CREATE {
    pub response: GmTicketResponse,
}

impl crate::Message for SMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0206;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // response: GmTicketResponse
        w.write_all(&(self.response.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // response: GmTicketResponse
        let response: GmTicketResponse = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GMTICKET_CREATE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_GMTICKET_CREATE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_GMTICKET_CREATE {}
