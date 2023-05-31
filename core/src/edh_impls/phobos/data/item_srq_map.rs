use std::collections::HashMap;

use crate::{defs::ReeInt, edh_impls::phobos::fsd::FsdMerge, edt};

pub(in super::super) type ItemSkillMap = HashMap<ReeInt, ReeInt>;
impl FsdMerge<edt::ItemSkillReq> for ItemSkillMap {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::ItemSkillReq> {
        self.into_iter()
            .map(|(sid, lvl)| edt::ItemSkillReq::new(id, sid, lvl))
            .collect()
    }
}
