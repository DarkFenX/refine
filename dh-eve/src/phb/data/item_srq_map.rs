use std::collections::HashMap;

use crate::phb::fsd::{FsdId, FsdMerge};

pub(in crate::phb) type PItemSkillMap = HashMap<rc::ItemId, rc::SkillLevel>;
impl FsdMerge<rc::ed::EItemSkillReq> for PItemSkillMap {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemSkillReq> {
        self.into_iter()
            .map(|(sid, lvl)| rc::ed::EItemSkillReq::new(id, sid, lvl))
            .collect()
    }
}
