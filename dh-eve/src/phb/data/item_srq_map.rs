use std::collections::HashMap;

use crate::phb::fsd::FsdMerge;

pub(in crate::phb) type PItemSkillMap = HashMap<rc::ReeInt, rc::ReeInt>;
impl FsdMerge<rc::ed::EItemSkillReq> for PItemSkillMap {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EItemSkillReq> {
        self.into_iter()
            .map(|(sid, lvl)| rc::ed::EItemSkillReq::new(id, sid, lvl))
            .collect()
    }
}
