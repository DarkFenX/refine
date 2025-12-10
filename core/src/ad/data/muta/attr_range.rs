use crate::ad::AAttrVal;

#[derive(Copy, Clone)]
pub struct AMutaAttrRange {
    pub min_mult: AAttrVal,
    pub max_mult: AAttrVal,
}
