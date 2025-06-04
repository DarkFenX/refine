use std::collections::HashMap;

use crate::shared::HSkillLevel;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::val) struct HValSrqFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, HashMap<rc::ItemTypeId, HValSrqSkillInfo>>,
}
impl From<&rc::val::ValSrqFail> for HValSrqFail {
    fn from(core_val_fail: &rc::val::ValSrqFail) -> Self {
        Self {
            items: core_val_fail
                .items
                .iter()
                .map(|(item_id, item_info)| {
                    (
                        *item_id,
                        item_info
                            .iter()
                            .map(|(skill_type_id, skill_info)| (*skill_type_id, skill_info.into()))
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::val) struct HValSrqSkillInfo {
    current_lvl: Option<HSkillLevel>,
    required_lvl: HSkillLevel,
}
impl From<&rc::val::ValSrqSkillInfo> for HValSrqSkillInfo {
    fn from(core_val_skill: &rc::val::ValSrqSkillInfo) -> Self {
        Self {
            current_lvl: core_val_skill
                .current_lvl
                .map(|core_skill_level| core_skill_level.get_inner()),
            required_lvl: core_val_skill.required_lvl.get_inner(),
        }
    }
}
