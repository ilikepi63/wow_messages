use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm:54`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm#L54):
/// ```text
/// struct QuestGiverReward {
///     u32 item;
///     u32 item_count;
///     u32 display_id;
/// }
/// ```
pub struct QuestGiverReward {
    pub item: u32,
    pub item_count: u32,
    pub display_id: u32,
}

impl QuestGiverReward {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // display_id: u32
        w.write_all(&self.display_id.to_le_bytes())?;

        Ok(())
    }
}

impl QuestGiverReward {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            item_count,
            display_id,
        })
    }

}
