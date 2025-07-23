use std::collections::HashMap;

use crate::phb::fsd::{FsdId, FsdMerge};

pub(in crate::phb) type PItemSkillMap = HashMap<rc::ed::EItemId, rc::ed::ESkillLevel>;
impl FsdMerge<rc::ed::EItemSkillReq> for PItemSkillMap {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemSkillReq> {
        self.into_iter()
            .map(|(sid, lvl)| rc::ed::EItemSkillReq {
                item_id: id,
                skill_id: sid,
                level: lvl,
            })
            .collect()
    }
}
