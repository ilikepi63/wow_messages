use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:244`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L244):
/// ```text
/// struct ItemSocket {
///     u32 color;
///     u32 content;
/// }
/// ```
pub struct ItemSocket {
    pub color: u32,
    pub content: u32,
}

impl ItemSocket {
    pub fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // color: u32
        w.write_all(&self.color.to_le_bytes())?;

        // content: u32
        w.write_all(&self.content.to_le_bytes())?;

        Ok(())
    }
}

impl ItemSocket {
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // color: u32
        let color = crate::util::read_u32_le(r)?;

        // content: u32
        let content = crate::util::read_u32_le(r)?;

        Ok(Self {
            color,
            content,
        })
    }

}
