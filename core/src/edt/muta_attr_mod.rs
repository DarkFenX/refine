use crate::{
    defs::{ReeFloat, ReeInt},
    util::Named,
};

/// EVE mutaplasmid attribute modification data.
#[derive(Debug)]
pub struct EMutaAttrMod {
    /// Mutaplasmid item type ID.
    pub muta_id: ReeInt,
    /// Refers an attribute being modified by the mutaplasmid.
    pub attr_id: ReeInt,
    /// Lower boundary of the modification range.
    pub min_attr_mult: ReeFloat,
    /// Upper boundary of the modification range.
    pub max_attr_mult: ReeFloat,
}
impl EMutaAttrMod {
    /// Make a new EVE mutaplasmid attribute conversion.
    pub fn new(muta_id: ReeInt, attr_id: ReeInt, min_attr_mult: ReeFloat, max_attr_mult: ReeFloat) -> Self {
        Self {
            muta_id,
            attr_id,
            min_attr_mult,
            max_attr_mult,
        }
    }
}
impl Named for EMutaAttrMod {
    fn get_name() -> &'static str {
        "edt::EMutaAttrMod"
    }
}
