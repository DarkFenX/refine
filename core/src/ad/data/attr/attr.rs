use crate::ad::{AAttrId, AValue};

pub struct AAttr {
    pub id: AAttrId,
    pub penalizable: bool,
    pub hig: bool,
    pub def_val: AValue,
    pub min_attr_id: Option<AAttrId> = None,
    pub max_attr_id: Option<AAttrId> = None,
}
