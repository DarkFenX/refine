use std::collections::HashMap;

use crate::fsd::FsdMerge;

pub(crate) type ItemSkillMap = HashMap<rc::ReeInt, rc::ReeInt>;
impl FsdMerge<rc::edt::EItemSkillReq> for ItemSkillMap {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::EItemSkillReq> {
        self.into_iter()
            .map(|(sid, lvl)| rc::edt::EItemSkillReq::new(id, sid, lvl))
            .collect()
    }
}
