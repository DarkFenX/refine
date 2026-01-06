use crate::{
    ed::{EAttrId, EFloat, EItemId},
    util::LibNamed,
};

pub struct EItemAttr {
    pub item_id: EItemId,
    pub attr_id: EAttrId,
    pub value: EFloat,
}
impl LibNamed for EItemAttr {
    fn lib_get_name() -> &'static str {
        "EItemAttr"
    }
}
