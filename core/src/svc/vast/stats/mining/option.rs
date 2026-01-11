use crate::ud::UItem;

/// Items which will be included in mining stats.
#[derive(Copy, Clone)]
pub struct StatMiningItemKinds {
    pub module: bool,
    pub minion: bool,
}
impl StatMiningItemKinds {
    /// Include all item types in mining stats.
    pub fn all_enabled() -> Self {
        Self {
            module: true,
            minion: true,
        }
    }
    /// Exclude all item types from mining stats.
    pub fn all_disabled() -> Self {
        Self {
            module: false,
            minion: false,
        }
    }
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem) -> bool {
        match u_item {
            UItem::Drone(_) => self.minion,
            UItem::Module(_) => self.module,
            _ => false,
        }
    }
}
