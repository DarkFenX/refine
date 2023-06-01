use std::collections::HashMap;

use crate::fsd::FsdMerge;

pub(crate) type ItemSkillMap = HashMap<rc::ReeInt, rc::ReeInt>;
impl FsdMerge<rc::edt::ItemSkillReq> for ItemSkillMap {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::ItemSkillReq> {
        self.into_iter()
            .map(|(sid, lvl)| rc::edt::ItemSkillReq::new(id, sid, lvl))
            .collect()
    }
}
