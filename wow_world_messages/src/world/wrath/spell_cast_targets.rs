use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::Vector3d;
use crate::world::wrath::SpellCastTargetFlags;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:154`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L154):
/// ```text
/// struct SpellCastTargets {
///     SpellCastTargetFlags target_flags;
///     if (target_flags & UNIT) {
///         PackedGuid unit_target;
///     }
///     if (target_flags & ITEM) {
///         PackedGuid item_target;
///     }
///     if (target_flags & SOURCE_LOCATION) {
///         Vector3d source;
///     }
///     if (target_flags & DEST_LOCATION) {
///         Vector3d destination;
///     }
///     if (target_flags & STRING) {
///         CString target_string;
///     }
/// }
/// ```
pub struct SpellCastTargets {
    pub target_flags: SpellCastTargets_SpellCastTargetFlags,
}

impl SpellCastTargets {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target_flags: SpellCastTargetFlags
        w.write_all(&(self.target_flags.as_int() as u32).to_le_bytes())?;

        if let Some(if_statement) = &self.target_flags.unit {
            // unit_target: PackedGuid
            if_statement.unit_target.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.item {
            // item_target: PackedGuid
            if_statement.item_target.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.source_location {
            // source: Vector3d
            if_statement.source.write_into_vec(w)?;

        }

        if let Some(if_statement) = &self.target_flags.dest_location {
            // destination: Vector3d
            if_statement.destination.write_into_vec(w)?;

        }

        if let Some(if_statement) = &self.target_flags.string {
            // target_string: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(if_statement.target_string.as_bytes().iter().rev().next(), Some(&0_u8), "String `target_string` must not be null-terminated.");
            w.write_all(if_statement.target_string.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        Ok(())
    }
}

impl SpellCastTargets {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // target_flags: SpellCastTargetFlags
        let target_flags = SpellCastTargetFlags::new(crate::util::read_u32_le(r)?);

        let target_flags_UNIT = if target_flags.is_UNIT() {
            // unit_target: PackedGuid
            let unit_target = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit {
                unit_target,
            })
        }
        else {
            None
        };

        let target_flags_ITEM = if target_flags.is_ITEM() {
            // item_target: PackedGuid
            let item_target = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Item {
                item_target,
            })
        }
        else {
            None
        };

        let target_flags_SOURCE_LOCATION = if target_flags.is_SOURCE_LOCATION() {
            // source: Vector3d
            let source = Vector3d::read(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_SourceLocation {
                source,
            })
        }
        else {
            None
        };

        let target_flags_DEST_LOCATION = if target_flags.is_DEST_LOCATION() {
            // destination: Vector3d
            let destination = Vector3d::read(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_DestLocation {
                destination,
            })
        }
        else {
            None
        };

        let target_flags_STRING = if target_flags.is_STRING() {
            // target_string: CString
            let target_string = crate::util::read_c_string_to_vec(r)?;
            let target_string = String::from_utf8(target_string)?;

            Some(SpellCastTargets_SpellCastTargetFlags_String {
                target_string,
            })
        }
        else {
            None
        };

        let target_flags = SpellCastTargets_SpellCastTargetFlags {
            inner: target_flags.as_int(),
            unit: target_flags_UNIT,
            item: target_flags_ITEM,
            source_location: target_flags_SOURCE_LOCATION,
            dest_location: target_flags_DEST_LOCATION,
            string: target_flags_STRING,
        };

        Ok(Self {
            target_flags,
        })
    }

}

impl SpellCastTargets {
    pub(crate) fn size(&self) -> usize {
        self.target_flags.size() // target_flags: SpellCastTargets_SpellCastTargetFlags
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags {
    inner: u32,
    unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,
    item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,
    source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,
    dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,
    string: Option<SpellCastTargets_SpellCastTargetFlags_String>,
}

impl SpellCastTargets_SpellCastTargetFlags {
    pub const fn new(inner: u32, unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,string: Option<SpellCastTargets_SpellCastTargetFlags_String>,) -> Self {
        Self {
            inner,
            unit, 
            item, 
            source_location, 
            dest_location, 
            string, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.unit.is_none()
        && self.item.is_none()
        && self.source_location.is_none()
        && self.dest_location.is_none()
        && self.string.is_none()
    }

    pub const fn new_UNUSED1() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNUSED1,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNUSED1(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNUSED1;
        self
    }

    pub const fn get_UNUSED1(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNUSED1) != 0
    }

    pub fn clear_UNUSED1(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNUSED1.reverse_bits();
        self
    }

    pub const fn new_UNIT(unit: SpellCastTargets_SpellCastTargetFlags_Unit) -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT,
            unit: Some(unit),
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNIT(mut self, unit: SpellCastTargets_SpellCastTargetFlags_Unit) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT;
        self.unit = Some(unit);
        self
    }

    pub const fn get_UNIT(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Unit> {
        self.unit.as_ref()
    }

    pub fn clear_UNIT(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT.reverse_bits();
        self.unit = None;
        self
    }

    pub const fn new_UNIT_RAID() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_RAID,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNIT_RAID(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_RAID;
        self
    }

    pub const fn get_UNIT_RAID(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_RAID) != 0
    }

    pub fn clear_UNIT_RAID(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_RAID.reverse_bits();
        self
    }

    pub const fn new_UNIT_PARTY() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_PARTY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNIT_PARTY(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_PARTY;
        self
    }

    pub const fn get_UNIT_PARTY(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_PARTY) != 0
    }

    pub fn clear_UNIT_PARTY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_PARTY.reverse_bits();
        self
    }

    pub const fn new_ITEM(item: SpellCastTargets_SpellCastTargetFlags_Item) -> Self {
        Self {
            inner: SpellCastTargetFlags::ITEM,
            unit: None,
            item: Some(item),
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_ITEM(mut self, item: SpellCastTargets_SpellCastTargetFlags_Item) -> Self {
        self.inner |= SpellCastTargetFlags::ITEM;
        self.item = Some(item);
        self
    }

    pub const fn get_ITEM(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Item> {
        self.item.as_ref()
    }

    pub fn clear_ITEM(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::ITEM.reverse_bits();
        self.item = None;
        self
    }

    pub const fn new_SOURCE_LOCATION(source_location: SpellCastTargets_SpellCastTargetFlags_SourceLocation) -> Self {
        Self {
            inner: SpellCastTargetFlags::SOURCE_LOCATION,
            unit: None,
            item: None,
            source_location: Some(source_location),
            dest_location: None,
            string: None,
        }
    }

    pub fn set_SOURCE_LOCATION(mut self, source_location: SpellCastTargets_SpellCastTargetFlags_SourceLocation) -> Self {
        self.inner |= SpellCastTargetFlags::SOURCE_LOCATION;
        self.source_location = Some(source_location);
        self
    }

    pub const fn get_SOURCE_LOCATION(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_SourceLocation> {
        self.source_location.as_ref()
    }

    pub fn clear_SOURCE_LOCATION(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::SOURCE_LOCATION.reverse_bits();
        self.source_location = None;
        self
    }

    pub const fn new_DEST_LOCATION(dest_location: SpellCastTargets_SpellCastTargetFlags_DestLocation) -> Self {
        Self {
            inner: SpellCastTargetFlags::DEST_LOCATION,
            unit: None,
            item: None,
            source_location: None,
            dest_location: Some(dest_location),
            string: None,
        }
    }

    pub fn set_DEST_LOCATION(mut self, dest_location: SpellCastTargets_SpellCastTargetFlags_DestLocation) -> Self {
        self.inner |= SpellCastTargetFlags::DEST_LOCATION;
        self.dest_location = Some(dest_location);
        self
    }

    pub const fn get_DEST_LOCATION(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_DestLocation> {
        self.dest_location.as_ref()
    }

    pub fn clear_DEST_LOCATION(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::DEST_LOCATION.reverse_bits();
        self.dest_location = None;
        self
    }

    pub const fn new_UNIT_ENEMY() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_ENEMY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNIT_ENEMY(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ENEMY;
        self
    }

    pub const fn get_UNIT_ENEMY(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_ENEMY) != 0
    }

    pub fn clear_UNIT_ENEMY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ENEMY.reverse_bits();
        self
    }

    pub const fn new_UNIT_ALLY() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_ALLY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNIT_ALLY(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ALLY;
        self
    }

    pub const fn get_UNIT_ALLY(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_ALLY) != 0
    }

    pub fn clear_UNIT_ALLY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ALLY.reverse_bits();
        self
    }

    pub const fn new_CORPSE_ENEMY() -> Self {
        Self {
            inner: SpellCastTargetFlags::CORPSE_ENEMY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_CORPSE_ENEMY(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::CORPSE_ENEMY;
        self
    }

    pub const fn get_CORPSE_ENEMY(&self) -> bool {
        (self.inner & SpellCastTargetFlags::CORPSE_ENEMY) != 0
    }

    pub fn clear_CORPSE_ENEMY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::CORPSE_ENEMY.reverse_bits();
        self
    }

    pub const fn new_UNIT_DEAD() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_DEAD,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNIT_DEAD(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_DEAD;
        self
    }

    pub const fn get_UNIT_DEAD(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_DEAD) != 0
    }

    pub fn clear_UNIT_DEAD(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_DEAD.reverse_bits();
        self
    }

    pub const fn new_GAMEOBJECT() -> Self {
        Self {
            inner: SpellCastTargetFlags::GAMEOBJECT,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_GAMEOBJECT(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::GAMEOBJECT;
        self
    }

    pub const fn get_GAMEOBJECT(&self) -> bool {
        (self.inner & SpellCastTargetFlags::GAMEOBJECT) != 0
    }

    pub fn clear_GAMEOBJECT(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::GAMEOBJECT.reverse_bits();
        self
    }

    pub const fn new_TRADE_ITEM() -> Self {
        Self {
            inner: SpellCastTargetFlags::TRADE_ITEM,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_TRADE_ITEM(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::TRADE_ITEM;
        self
    }

    pub const fn get_TRADE_ITEM(&self) -> bool {
        (self.inner & SpellCastTargetFlags::TRADE_ITEM) != 0
    }

    pub fn clear_TRADE_ITEM(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::TRADE_ITEM.reverse_bits();
        self
    }

    pub const fn new_STRING(string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
        Self {
            inner: SpellCastTargetFlags::STRING,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: Some(string),
        }
    }

    pub fn set_STRING(mut self, string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
        self.inner |= SpellCastTargetFlags::STRING;
        self.string = Some(string);
        self
    }

    pub const fn get_STRING(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_String> {
        self.string.as_ref()
    }

    pub fn clear_STRING(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::STRING.reverse_bits();
        self.string = None;
        self
    }

    pub const fn new_LOCKED() -> Self {
        Self {
            inner: SpellCastTargetFlags::LOCKED,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_LOCKED(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::LOCKED;
        self
    }

    pub const fn get_LOCKED(&self) -> bool {
        (self.inner & SpellCastTargetFlags::LOCKED) != 0
    }

    pub fn clear_LOCKED(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::LOCKED.reverse_bits();
        self
    }

    pub const fn new_CORPSE_ALLY() -> Self {
        Self {
            inner: SpellCastTargetFlags::CORPSE_ALLY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_CORPSE_ALLY(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::CORPSE_ALLY;
        self
    }

    pub const fn get_CORPSE_ALLY(&self) -> bool {
        (self.inner & SpellCastTargetFlags::CORPSE_ALLY) != 0
    }

    pub fn clear_CORPSE_ALLY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::CORPSE_ALLY.reverse_bits();
        self
    }

    pub const fn new_UNIT_MINIPET() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_MINIPET,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNIT_MINIPET(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_MINIPET;
        self
    }

    pub const fn get_UNIT_MINIPET(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_MINIPET) != 0
    }

    pub fn clear_UNIT_MINIPET(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_MINIPET.reverse_bits();
        self
    }

    pub const fn new_GLYPH_SLOT() -> Self {
        Self {
            inner: SpellCastTargetFlags::GLYPH_SLOT,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_GLYPH_SLOT(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::GLYPH_SLOT;
        self
    }

    pub const fn get_GLYPH_SLOT(&self) -> bool {
        (self.inner & SpellCastTargetFlags::GLYPH_SLOT) != 0
    }

    pub fn clear_GLYPH_SLOT(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::GLYPH_SLOT.reverse_bits();
        self
    }

    pub const fn new_DEST_TARGET() -> Self {
        Self {
            inner: SpellCastTargetFlags::DEST_TARGET,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_DEST_TARGET(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::DEST_TARGET;
        self
    }

    pub const fn get_DEST_TARGET(&self) -> bool {
        (self.inner & SpellCastTargetFlags::DEST_TARGET) != 0
    }

    pub fn clear_DEST_TARGET(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::DEST_TARGET.reverse_bits();
        self
    }

    pub const fn new_UNUSED20() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNUSED20,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNUSED20(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNUSED20;
        self
    }

    pub const fn get_UNUSED20(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNUSED20) != 0
    }

    pub fn clear_UNUSED20(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNUSED20.reverse_bits();
        self
    }

    pub const fn new_UNIT_PASSENGER() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_PASSENGER,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub fn set_UNIT_PASSENGER(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_PASSENGER;
        self
    }

    pub const fn get_UNIT_PASSENGER(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_PASSENGER) != 0
    }

    pub fn clear_UNIT_PASSENGER(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_PASSENGER.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SpellCastTargets_SpellCastTargetFlags {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.unit {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.item {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.source_location {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.dest_location {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.string {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_Unit {
    pub unit_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_Unit {
    pub(crate) fn size(&self) -> usize {
        self.unit_target.size() // unit_target: Guid
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_Item {
    pub item_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_Item {
    pub(crate) fn size(&self) -> usize {
        self.item_target.size() // item_target: Guid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_SourceLocation {
    pub source: Vector3d,
}

impl SpellCastTargets_SpellCastTargetFlags_SourceLocation {
    pub(crate) fn size(&self) -> usize {
        12 // source: Vector3d
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub destination: Vector3d,
}

impl SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub(crate) fn size(&self) -> usize {
        12 // destination: Vector3d
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_String {
    pub target_string: String,
}

impl SpellCastTargets_SpellCastTargetFlags_String {
    pub(crate) fn size(&self) -> usize {
        self.target_string.len() + 1 // target_string: CString
    }
}
