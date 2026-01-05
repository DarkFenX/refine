use crate::{
    ed::{EAttrId, EGenFloat, EItemId},
    util::LibNamed,
};

pub struct EMutaAttrMod {
    pub muta_id: EItemId,
    pub attr_id: EAttrId,
    pub min_attr_mult: EGenFloat,
    pub max_attr_mult: EGenFloat,
}
impl LibNamed for EMutaAttrMod {
    fn lib_get_name() -> &'static str {
        "EMutaAttrMod"
    }
}
