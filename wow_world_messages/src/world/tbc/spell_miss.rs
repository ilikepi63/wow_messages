use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::SpellMissInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm:49`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm#L49):
/// ```text
/// struct SpellMiss {
///     Guid target_guid;
///     SpellMissInfo miss_info;
///     if (miss_info == REFLECT) {
///         u8 reflect_result;
///     }
/// }
/// ```
pub struct SpellMiss {
    pub target_guid: Guid,
    pub miss_info: SpellMiss_SpellMissInfo,
}

impl SpellMiss {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int() as u32).to_le_bytes())?;

        match &self.miss_info {
            SpellMiss_SpellMissInfo::None => {}
            SpellMiss_SpellMissInfo::Miss => {}
            SpellMiss_SpellMissInfo::Resist => {}
            SpellMiss_SpellMissInfo::Dodge => {}
            SpellMiss_SpellMissInfo::Parry => {}
            SpellMiss_SpellMissInfo::Block => {}
            SpellMiss_SpellMissInfo::Evade => {}
            SpellMiss_SpellMissInfo::Immune => {}
            SpellMiss_SpellMissInfo::Immune2 => {}
            SpellMiss_SpellMissInfo::Deflect => {}
            SpellMiss_SpellMissInfo::Absorb => {}
            SpellMiss_SpellMissInfo::Reflect {
                reflect_result,
            } => {
                // reflect_result: u8
                w.write_all(&reflect_result.to_le_bytes())?;

            }
        }

        Ok(())
    }
}

impl SpellMiss {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // miss_info: SpellMissInfo
        let miss_info: SpellMissInfo = crate::util::read_u32_le(r)?.try_into()?;

        let miss_info_if = match miss_info {
            SpellMissInfo::None => SpellMiss_SpellMissInfo::None,
            SpellMissInfo::Miss => SpellMiss_SpellMissInfo::Miss,
            SpellMissInfo::Resist => SpellMiss_SpellMissInfo::Resist,
            SpellMissInfo::Dodge => SpellMiss_SpellMissInfo::Dodge,
            SpellMissInfo::Parry => SpellMiss_SpellMissInfo::Parry,
            SpellMissInfo::Block => SpellMiss_SpellMissInfo::Block,
            SpellMissInfo::Evade => SpellMiss_SpellMissInfo::Evade,
            SpellMissInfo::Immune => SpellMiss_SpellMissInfo::Immune,
            SpellMissInfo::Immune2 => SpellMiss_SpellMissInfo::Immune2,
            SpellMissInfo::Deflect => SpellMiss_SpellMissInfo::Deflect,
            SpellMissInfo::Absorb => SpellMiss_SpellMissInfo::Absorb,
            SpellMissInfo::Reflect => {
                // reflect_result: u8
                let reflect_result = crate::util::read_u8_le(r)?;

                SpellMiss_SpellMissInfo::Reflect {
                    reflect_result,
                }
            }
        };

        Ok(Self {
            target_guid,
            miss_info: miss_info_if,
        })
    }

}

impl SpellMiss {
    pub(crate) fn size(&self) -> usize {
        8 // target_guid: Guid
        + self.miss_info.size() // miss_info: SpellMiss_SpellMissInfo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellMiss_SpellMissInfo {
    None,
    Miss,
    Resist,
    Dodge,
    Parry,
    Block,
    Evade,
    Immune,
    Immune2,
    Deflect,
    Absorb,
    Reflect {
        reflect_result: u8,
    },
}

impl Default for SpellMiss_SpellMissInfo {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SpellMiss_SpellMissInfo {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Miss => 1,
            Self::Resist => 2,
            Self::Dodge => 3,
            Self::Parry => 4,
            Self::Block => 5,
            Self::Evade => 6,
            Self::Immune => 7,
            Self::Immune2 => 8,
            Self::Deflect => 9,
            Self::Absorb => 10,
            Self::Reflect { .. } => 11,
        }
    }

}

impl SpellMiss_SpellMissInfo {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                4
            }
            Self::Miss => {
                4
            }
            Self::Resist => {
                4
            }
            Self::Dodge => {
                4
            }
            Self::Parry => {
                4
            }
            Self::Block => {
                4
            }
            Self::Evade => {
                4
            }
            Self::Immune => {
                4
            }
            Self::Immune2 => {
                4
            }
            Self::Deflect => {
                4
            }
            Self::Absorb => {
                4
            }
            Self::Reflect {
                reflect_result,
            } => {
                4
                + 1 // reflect_result: u8
            }
        }
    }
}
