use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValSrqFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HashMap<rc::EItemId, HValSrqSkillInfo>>,
}
impl HValSrqFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolValSrqFail>> for HValSrqFail {
    fn from(core_val_fails: &Vec<rc::SolValSrqFail>) -> Self {
        Self {
            data: core_val_fails
                .iter()
                .map(|v| {
                    (
                        v.item_id,
                        v.skills.iter().map(|s| (s.skill_type_id, s.into())).collect(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValSrqSkillInfo {
    skill_lvl: Option<rc::SkillLevel>,
    req_lvl: rc::SkillLevel,
}
impl From<&rc::SolValSrqSkillInfo> for HValSrqSkillInfo {
    fn from(core_val_skill: &rc::SolValSrqSkillInfo) -> Self {
        Self {
            skill_lvl: core_val_skill.skill_lvl,
            req_lvl: core_val_skill.req_lvl,
        }
    }
}
