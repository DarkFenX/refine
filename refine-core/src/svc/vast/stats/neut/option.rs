use crate::{misc::DefOption, nd::NEffectNeutKind, rd::REffect};

/// Items which will be included in neut stats.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatNeutItemKinds {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::kind_default"))]
    pub default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    pub module: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minion: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bomb: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub side_effect: DefOption = DefOption::Default,
}
impl StatNeutItemKinds {
    pub(in crate::svc::vast) fn resolve(&self, r_effect: &REffect) -> bool {
        let neut_kind = match &r_effect.neut {
            Some(neut) => neut.kind,
            None => return false,
        };
        match neut_kind {
            NEffectNeutKind::Module => self.module.is_enabled(self.default),
            NEffectNeutKind::Minion => self.minion.is_enabled(self.default),
            NEffectNeutKind::Bomb => self.bomb.is_enabled(self.default),
            NEffectNeutKind::SideEffect => self.side_effect.is_enabled(self.default),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    pub(super) fn kind_default() -> bool {
        true
    }
}
