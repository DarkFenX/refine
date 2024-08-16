use crate::defs::EAttrId;

/// Effect-specific charge information.
#[derive(Copy, Clone)]
pub struct AEffectChargeInfo {
    /// Specifies where effect should look for a charge.
    pub location: AEffectChargeLocation,
    /// Defines if charge is powered by the effect, i.e. if running the effect and targeting it
    /// somewhere forces charge effects to follow.
    pub powered: bool,
}
impl AEffectChargeInfo {
    pub(crate) fn new(location: AEffectChargeLocation, run_effects: bool) -> Self {
        Self { location, powered: run_effects }
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
