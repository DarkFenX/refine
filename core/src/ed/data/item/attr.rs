use crate::{
    ed::{EAttrId, EGenFloat, EItemId},
    util::LibNamed,
};

pub struct EItemAttr {
    pub item_id: EItemId,
    pub attr_id: EAttrId,
    pub value: EGenFloat,
}
impl LibNamed for EItemAttr {
    fn lib_get_name() -> &'static str {
        "EItemAttr"
    }
}
