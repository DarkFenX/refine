use crate::{DefOption, ud::UItem};

/// Items which will be included in mining stats.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatMiningItemKinds {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::kind_default"))]
    default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    module: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    minion: DefOption = DefOption::Default,
}
const impl Default for StatMiningItemKinds {
    fn default() -> Self {
        Self { .. }
    }
}
impl StatMiningItemKinds {
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
}

/// Mining stats depend on what kind of resource is targeted. Regular resources are prone to mining
/// residue and crits, while mission resources have neither.
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum StatMiningResourceKind {
    Regular,
    Mission,
}
const impl Default for StatMiningResourceKind {
    fn default() -> Self {
        Self::Regular
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatMiningItemKinds {
    pub(in crate::svc::vast::stats) fn resolve(&self, u_item: &UItem) -> bool {
        match u_item {
            UItem::Drone(..) => self.minion.is_enabled(self.default),
            UItem::Module(..) => self.module.is_enabled(self.default),
            _ => false,
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
