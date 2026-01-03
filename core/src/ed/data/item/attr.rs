use crate::{
    ed::{EAttrId, EGenFloat, EItemId},
    util::Named,
};

pub struct EItemAttr {
    pub item_id: EItemId,
    pub attr_id: EAttrId,
    pub value: EGenFloat,
}
impl Named for EItemAttr {
    fn get_name() -> &'static str {
        "EItemAttr"
    }
}
