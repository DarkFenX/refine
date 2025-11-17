use crate::{
    ad::{AAttrId, AAttrVal},
    util::Named,
};

pub struct AAttr {
    pub id: AAttrId,
    pub penalizable: bool,
    pub hig: bool,
    pub def_val: AAttrVal,
    pub min_attr_id: Option<AAttrId> = None,
    pub max_attr_id: Option<AAttrId> = None,
}
impl Named for AAttr {
    fn get_name() -> &'static str {
        "AAttr"
    }
}
