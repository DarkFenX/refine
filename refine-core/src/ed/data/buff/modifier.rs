use crate::ed::{EAttrId, EItemGrpId, EItemId};

pub struct EBuffIm {
    pub attr_id: EAttrId,
}

pub struct EBuffLm {
    pub attr_id: EAttrId,
}

pub struct EBuffLgm {
    pub attr_id: EAttrId,
    pub group_id: EItemGrpId,
}

pub struct EBuffLrsm {
    pub attr_id: EAttrId,
    pub skill_id: EItemId,
}
