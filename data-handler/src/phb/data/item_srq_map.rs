use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::phb::fsd::{FsdId, FsdMerge};

#[serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::phb) struct PItemSkillMap {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    data: Vec<(i32, i32)>,
}
impl FsdMerge<rc::ed::EItemSkillReq> for PItemSkillMap {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemSkillReq> {
        self.data
            .into_iter()
            .map(|(sid, lvl)| rc::ed::EItemSkillReq {
                item_id: rc::ed::EItemId::from_i32(id),
                skill_id: rc::ed::EItemId::from_i32(sid),
                level: rc::ed::EInt::from_i32(lvl),
            })
            .collect()
    }
}
