use crate::{DefOption, ud::UItem};

/// Items which will be included in outgoing rep stats.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOutRepItemKinds {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::kind_default"))]
    pub default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    pub module: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minion: DefOption = DefOption::Default,
}
impl StatOutRepItemKinds {
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem) -> bool {
        match u_item {
            UItem::Drone(..) => self.minion.is_enabled(self.default),
            UItem::Fighter(..) => self.minion.is_enabled(self.default),
            // Just consider everything else as modules
            _ => self.module.is_enabled(self.default),
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
