use crate::{
    ed::{EAttrId, EAttrVal, EItemId},
    util::Named,
};

pub struct EMutaAttrMod {
    pub muta_id: EItemId,
    pub attr_id: EAttrId,
    pub min_attr_mult: EAttrVal,
    pub max_attr_mult: EAttrVal,
}
impl Named for EMutaAttrMod {
    fn get_name() -> &'static str {
        "EMutaAttrMod"
    }
}
