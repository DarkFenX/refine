use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct SkillInfo {
    pub id: rc::ItemId,
    pub extended: Option<SkillInfoExt>,
}

pub struct SkillInfoExt {
    pub kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub level: rc::SkillLevel,
    pub state: bool,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
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
                    kind: rc::ItemKind::Skill,
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
