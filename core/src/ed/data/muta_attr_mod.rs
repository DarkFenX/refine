use crate::{
    ed::{EAttrId, EAttrVal, EItemId},
    util::Named,
};

/// EVE mutator (aka mutaplasmid) attribute modification data.
pub struct EMutaAttrMod {
    /// Mutator item type ID.
    pub muta_id: EItemId,
    /// Refers an attribute being modified by the mutator.
    pub attr_id: EAttrId,
    /// Lower boundary of the modification range.
    pub min_attr_mult: EAttrVal,
    /// Upper boundary of the modification range.
    pub max_attr_mult: EAttrVal,
}
impl Named for EMutaAttrMod {
    fn get_name() -> &'static str {
        "EMutaAttrMod"
    }
}
