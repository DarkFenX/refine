use crate::{
    ed::{EAttrId, EAttrUnitId, EAttrVal},
    util::Named,
};

pub struct EAttr {
    pub id: EAttrId,
    pub stackable: bool,
    pub high_is_good: bool,
    pub default_value: EAttrVal,
    pub min_attr_id: Option<EAttrId>,
    pub max_attr_id: Option<EAttrId>,
    pub unit_id: Option<EAttrUnitId>,
}
impl Named for EAttr {
    fn get_name() -> &'static str {
        "EAttr"
    }
}
