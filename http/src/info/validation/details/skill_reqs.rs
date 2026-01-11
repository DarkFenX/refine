use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValSrqFail {
    #[serde_as(as = "Map<DisplayFromStr, Map<DisplayFromStr, _>>")]
    items: Vec<(rc::ItemId, Vec<(i32, HValSrqSkillInfo)>)>,
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
                            .map(|(skill_type_id, skill_info)| (skill_type_id.into_i32(), skill_info.into()))
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Serialize_tuple)]
struct HValSrqSkillInfo {
    current_lvl: Option<u8>,
    required_lvl: u8,
}
impl From<&rc::val::ValSrqSkillInfo> for HValSrqSkillInfo {
    fn from(core_val_skill: &rc::val::ValSrqSkillInfo) -> Self {
        Self {
            current_lvl: core_val_skill
                .current_lvl
                .map(|core_skill_level| core_skill_level.into_u8()),
            required_lvl: core_val_skill.required_lvl.into_u8(),
        }
    }
}
