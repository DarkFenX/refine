use crate::{
    ed::{EAttrId, EGenFloat, EItemId},
    util::Named,
};

pub struct EMutaAttrMod {
    pub muta_id: EItemId,
    pub attr_id: EAttrId,
    pub min_attr_mult: EGenFloat,
    pub max_attr_mult: EGenFloat,
}
impl Named for EMutaAttrMod {
    fn get_name() -> &'static str {
        "EMutaAttrMod"
    }
}
