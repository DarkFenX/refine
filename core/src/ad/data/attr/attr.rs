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
impl std::fmt::Display for AAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
