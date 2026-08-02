use crate::{DefOption, ud::UItem};

/// Items which will be included in mining stats.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatMiningItemKinds {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::kind_default"))]
    pub default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    pub module: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minion: DefOption = DefOption::Default,
}
impl StatMiningItemKinds {
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem) -> bool {
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
