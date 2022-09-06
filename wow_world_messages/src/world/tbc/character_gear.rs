use std::convert::{TryFrom, TryInto};
use crate::world::tbc::InventoryType;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum_2_4_3.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum_2_4_3.wowm#L32):
/// ```text
/// struct CharacterGear {
///     u32 equipment_display_id;
///     InventoryType inventory_type;
///     u32 enchantment;
/// }
/// ```
pub struct CharacterGear {
    pub equipment_display_id: u32,
    pub inventory_type: InventoryType,
    pub enchantment: u32,
}

impl CharacterGear {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // equipment_display_id: u32
        w.write_all(&self.equipment_display_id.to_le_bytes())?;

        // inventory_type: InventoryType
        w.write_all(&(self.inventory_type.as_int() as u8).to_le_bytes())?;

        // enchantment: u32
        w.write_all(&self.enchantment.to_le_bytes())?;

        Ok(())
    }
}

impl CharacterGear {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::read_u32_le(r)?;

        // inventory_type: InventoryType
        let inventory_type: InventoryType = crate::util::read_u8_le(r)?.try_into()?;

        // enchantment: u32
        let enchantment = crate::util::read_u32_le(r)?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
            enchantment,
        })
    }

}
