use crate::{
    defs::{ReeFloat, ReeInt},
    util::Named,
};

/// Mutaplasmid attribute modification data.
#[derive(Debug)]
pub struct MutaAttrMod {
    /// Mutaplasmid item type ID.
    pub muta_id: ReeInt,
    /// Refers an attribute being modified by the mutaplasmid.
    pub attr_id: ReeInt,
    /// Lower boundary of the modification range.
    pub min_attr_mult: ReeFloat,
    /// Upper boundary of the modification range.
    pub max_attr_mult: ReeFloat,
}
impl MutaAttrMod {
    /// Make a new mutaplasmid attribute conversion.
    pub fn new(muta_id: ReeInt, attr_id: ReeInt, min_attr_mult: ReeFloat, max_attr_mult: ReeFloat) -> Self {
        Self {
            muta_id,
            attr_id,
            min_attr_mult,
            max_attr_mult,
        }
    }
}
impl Named for MutaAttrMod {
    fn get_name() -> &'static str {
        "edt::MutaAttrMod"
    }
}
