use crate::{
    ad::{AItemId, ASkillLevel},
    util::CMap,
};

pub struct AItemSkillReq {
    pub id: AItemId,
    pub level: ASkillLevel,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct AItemSkillReqs {
    data: CMap<AItemId, AItemSkillReq>,
}
impl AItemSkillReqs {
    pub const fn new() -> Self {
        Self {
            data: CMap::const_new(),
        }
    }
    pub fn insert(&mut self, val: AItemSkillReq) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemSkillReq> {
        self.data.values()
    }
}
impl FromIterator<AItemSkillReq> for AItemSkillReqs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemSkillReq>,
    {
        Self {
            data: CMap::const_from_iter(iter.into_iter().map(|v| (v.id, v))),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemSkillReqs {
    pub(crate) fn contains_id(&self, id: &AItemId) -> bool {
        self.data.contains_key(id)
    }
}
