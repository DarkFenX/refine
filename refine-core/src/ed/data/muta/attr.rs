use crate::ed::{EAttrId, EFloat};

pub struct EMutaAttr {
    pub attr_id: EAttrId,
    pub min_attr_mult: EFloat,
    pub max_attr_mult: EFloat,
}
