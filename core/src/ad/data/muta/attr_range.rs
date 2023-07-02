use crate::defs::AttrVal;

/// Stores mutation range of specific attribute of specific mutaplasmid.
pub struct AMutaAttrRange {
    /// Lower boundary of the modification range.
    pub min_mult: AttrVal,
    /// Upper boundary of the modification range.
    pub max_mult: AttrVal,
}
impl AMutaAttrRange {
    /// Make a new attribute mutation range.
    pub(crate) fn new(min_mult: AttrVal, max_mult: AttrVal) -> Self {
        Self { min_mult, max_mult }
    }
}
