use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::phb::parsing::{Key, KeyMerge};

#[serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::phb) struct PItemSkillMap {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    data: Vec<(i32, i32)>,
}
impl KeyMerge<rc::ed::EItemSkillReq> for PItemSkillMap {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItemSkillReq> {
        self.data
            .into_iter()
            .map(|(skill_id, lvl)| rc::ed::EItemSkillReq {
                item_id: rc::ed::EItemId::from_i32(key),
                skill_id: rc::ed::EItemId::from_i32(skill_id),
                level: rc::ed::EInt::from_i32(lvl),
            })
            .collect()
    }
}
