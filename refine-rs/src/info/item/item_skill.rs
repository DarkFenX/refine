use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification, SkillLevel,
};

pub struct SkillInfo {
    pub id: ItemId,
    pub extended: Option<SkillInfoExt>,
}

pub struct SkillInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub level: SkillLevel,
    pub state: bool,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillInfo {
    pub(in crate::info) fn from_core(core_skill: &mut rc::SkillMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_skill.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(SkillInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Skill,
                    type_id: core_skill.get_type_id(),
                    fit_id: core_skill.get_fit().get_fit_id(),
                    level: core_skill.get_level(),
                    state: core_skill.get_state(),
                    attrs: get_attrs(core_skill, item_mode),
                    effects: get_effects(core_skill, item_mode),
                    mods: get_mods(core_skill, item_mode),
                }),
            },
        }
    }
}
