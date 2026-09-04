use rc::ItemCommon;

use super::shared::{get_attrs, get_effect_mode_overrides, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, EffectId, EffectMode, FitId, ItemAttrValues, ItemEffectInfo, ItemId, ItemInfoMode, ItemTypeId,
    Modification, SkillLevel, shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SkillInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SkillInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct SkillInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub level: SkillLevel,
    pub state: bool,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effect_mode_overrides: Vec<(EffectId, EffectMode)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, ItemAttrValues)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<(EffectId, ItemEffectInfo)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillInfo {
    pub(in crate::info) fn from_core(
        core_skill: &mut rc::SkillMut,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let skill_id = core_skill.get_item_id();
        let skill_info_mode = item_info_modes.get(&skill_id);
        Self {
            id: skill_id,
            extended: match skill_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(SkillInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Skill,
                    type_id: core_skill.get_type_id(),
                    fit_id: core_skill.get_fit().get_fit_id(),
                    level: core_skill.get_level(),
                    state: core_skill.get_state(),
                    effect_mode_overrides: get_effect_mode_overrides(core_skill, skill_info_mode),
                    attrs: get_attrs(core_skill, skill_info_mode),
                    effects: get_effects(core_skill, skill_info_mode),
                    mods: get_mods(core_skill, skill_info_mode),
                }),
            },
        }
    }
}
