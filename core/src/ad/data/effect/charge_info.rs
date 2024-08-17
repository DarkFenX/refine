use crate::defs::EAttrId;

/// Effect-specific charge information.
#[derive(Copy, Clone)]
pub enum AEffectChargeInfo {
    /// Effect uses charge loaded into item.
    Loaded,
    /// Effect uses charge referenced by an attribute on effect item.
    Attr(EAttrId),
}
