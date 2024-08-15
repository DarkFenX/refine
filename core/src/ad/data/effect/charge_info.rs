use crate::defs::EAttrId;

/// Effect-specific charge information.
#[derive(Copy, Clone)]
pub struct AEffectChargeInfo {
    /// Specifies where effect should look for a charge.
    pub location: AEffectChargeLocation,
    /// Defines if module should run charge effects when activated.
    pub run_effects: bool,
}
impl AEffectChargeInfo {
    pub(crate) fn new(location: AEffectChargeLocation, run_effects: bool) -> Self {
        Self { location, run_effects }
    }
}

/// Specifies where effect should look for a charge.
#[derive(Copy, Clone)]
pub enum AEffectChargeLocation {
    /// Effect uses charge loaded into item.
    Loaded,
    /// Effect uses charge referenced by an attribute on effect item.
    Attr(EAttrId),
}
