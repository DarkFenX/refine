use crate::ed::{EAttrId, EItemGrpId, EItemId};

pub struct EBuffIM {
    pub attr_id: EAttrId,
}

pub struct EBuffLM {
    pub attr_id: EAttrId,
}

pub struct EBuffLGM {
    pub attr_id: EAttrId,
    pub group_id: EItemGrpId,
}

pub struct EBuffLRSM {
    pub attr_id: EAttrId,
    pub skill_id: EItemId,
}
