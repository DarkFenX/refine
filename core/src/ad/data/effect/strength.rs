use crate::ad::{AAttrId, AValue};

#[derive(Copy, Clone, PartialEq)]
pub enum AEffectModStrength {
    Attr(AAttrId),
    Hardcoded(AValue),
}
