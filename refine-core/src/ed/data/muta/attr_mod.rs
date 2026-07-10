use crate::{
    ed::{EAttrId, EFloat, EItemId},
    util::LibNamed,
};

pub struct EMutaAttrMod {
    pub muta_id: EItemId,
    pub attr_id: EAttrId,
    pub min_attr_mult: EFloat,
    pub max_attr_mult: EFloat,
}
impl LibNamed for EMutaAttrMod {
    fn lib_get_name() -> &'static str {
        "EMutaAttrMod"
    }
}
