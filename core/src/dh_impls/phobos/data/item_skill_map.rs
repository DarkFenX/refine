use std::collections::HashMap;

use crate::defines::ReeInt;
use crate::dh;

use super::super::fsd::FsdMerge;

pub(in super::super) type ItemSkillMap = HashMap<ReeInt, ReeInt>;
impl FsdMerge<dh::ItemSkillReq> for ItemSkillMap {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemSkillReq> {
        self.into_iter()
            .map(|(sid, lvl)| dh::ItemSkillReq::new(id, sid, lvl))
            .collect()
    }
}
