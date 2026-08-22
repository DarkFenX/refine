use crate::{DefOption, nd::NEffectNeutKind, rd::REffect};

/// Items which will be included in neut stats.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatNeutItemKinds {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::kind_default"))]
    default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    module: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    minion: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    bomb: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    side_effect: DefOption = DefOption::Default,
}
const impl Default for StatNeutItemKinds {
    fn default() -> Self {
        Self { .. }
    }
}
impl StatNeutItemKinds {
    /// True to have all supported item kinds enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    pub fn with_module(mut self, enabled: bool) -> Self {
        self.module = enabled.into();
        self
    }
    pub fn with_minion(mut self, enabled: bool) -> Self {
        self.minion = enabled.into();
        self
    }
    pub fn with_bomb(mut self, enabled: bool) -> Self {
        self.bomb = enabled.into();
        self
    }
    pub fn with_side_effect(mut self, enabled: bool) -> Self {
        self.side_effect = enabled.into();
        self
    }
}
impl StatNeutItemKinds {
    pub(in crate::svc::vast::stats) fn resolve(&self, r_effect: &REffect) -> bool {
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
