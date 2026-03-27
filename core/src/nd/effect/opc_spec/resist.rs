use crate::ad::AAttrId;

pub(crate) enum NEffectResist {
    Standard,
    Attr(AAttrId),
}
