use std::convert::{TryFrom, TryInto};
use crate::world::wrath::BarberShopResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_barber_shop_result.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_barber_shop_result.wowm#L10):
/// ```text
/// smsg SMSG_BARBER_SHOP_RESULT = 0x0428 {
///     (u32)BarberShopResult result;
/// }
/// ```
pub struct SMSG_BARBER_SHOP_RESULT {
    pub result: BarberShopResult,
}

impl crate::Message for SMSG_BARBER_SHOP_RESULT {
    const OPCODE: u32 = 0x0428;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: BarberShopResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0428, size: body_size as u32 });
        }

        // result: BarberShopResult
        let result: BarberShopResult = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_BARBER_SHOP_RESULT {}
