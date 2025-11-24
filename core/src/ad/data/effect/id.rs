use crate::ad::{ACustomEffectId, ADogmaEffectId, AItemId};

/// ID of an adapted effect.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AEffectId {
    /// ID of a general EVE effect.
    Dogma(ADogmaEffectId),
    /// Space component effect attached to an item, system-wide effect part.
    ScSystemWide(AItemId),
    /// Space component effect attached to an item, system buff emitter part.
    ScSystemEmitter(AItemId),
    /// Space component effect attached to an item, proximity effect part.
    ScProxyEffect(AItemId),
    /// Space component effect attached to an item, proximity trap/trigger part.
    ScProxyTrap(AItemId),
    /// Space component effect attached to an item, ship link part.
    ScShipLink(AItemId),
    /// ID of an effect created by the library.
    Custom(ACustomEffectId),
}
impl std::fmt::Display for AEffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dogma(id) => write!(f, "d{id}"),
            Self::ScSystemWide(id) => write!(f, "scsw{id}"),
            Self::ScSystemEmitter(id) => write!(f, "scse{id}"),
            Self::ScProxyEffect(id) => write!(f, "scpe{id}"),
            Self::ScProxyTrap(id) => write!(f, "scpt{id}"),
            Self::ScShipLink(id) => write!(f, "scsl{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}
