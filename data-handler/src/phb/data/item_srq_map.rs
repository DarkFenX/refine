use std::collections::HashMap;

use crate::phb::fsd::{FsdId, FsdMerge};

pub(in crate::phb) type PItemSkillMap = HashMap<i32, i32>;
impl FsdMerge<rc::ed::EItemSkillReq> for PItemSkillMap {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemSkillReq> {
        self.into_iter()
            .map(|(sid, lvl)| rc::ed::EItemSkillReq {
                item_id: rc::ed::EItemId::from_i32(id),
                skill_id: rc::ed::EItemId::from_i32(sid),
                level: rc::ed::EInt::from_i32(lvl),
            })
            .collect()
    }
}
