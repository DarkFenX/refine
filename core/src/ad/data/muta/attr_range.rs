use crate::ad::AAttrVal;

/// Stores mutation range of specific attribute of specific mutator.
pub struct AMutaAttrRange {
    /// Lower boundary of the modification range.
    pub min_mult: AAttrVal,
    /// Upper boundary of the modification range.
    pub max_mult: AAttrVal,
}
