use crate::ad::AAttrId;

pub(crate) enum NEffectResist {
    // On-effect reference to resist attr ID, or, if it is not defined, on-item reference from the
    // standard remoteResistanceID attribute
    Standard,
    // Defines attribute whose value will have reference to resistance attribute ID
    AttrRef(AAttrId),
}
