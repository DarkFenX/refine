use crate::ad::AAttrId;

pub(crate) enum NEffectDuration {
    Effect,
    AttrMs(AAttrId),
}
