use crate::{
    ed::{EAttrId, EAttrVal, EItemId},
    util::Named,
};

pub struct EItemAttr {
    pub item_id: EItemId,
    pub attr_id: EAttrId,
    pub value: EAttrVal,
}
impl Named for EItemAttr {
    fn get_name() -> &'static str {
        "EItemAttr"
    }
}
