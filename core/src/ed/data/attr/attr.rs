use crate::{
    ed::{EAttrId, EAttrUnitId, EGenFloat},
    util::LibNamed,
};

pub struct EAttr {
    pub id: EAttrId,
    pub stackable: bool,
    pub high_is_good: bool,
    pub default_value: EGenFloat,
    pub min_attr_id: Option<EAttrId>,
    pub max_attr_id: Option<EAttrId>,
    pub unit_id: Option<EAttrUnitId>,
}
impl LibNamed for EAttr {
    fn lib_get_name() -> &'static str {
        "EAttr"
    }
}
