use crate::ad::AValue;

#[derive(Copy, Clone)]
pub struct AMutaAttrRange {
    pub min_mult: AValue,
    pub max_mult: AValue,
}
