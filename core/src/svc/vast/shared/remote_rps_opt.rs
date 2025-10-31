use crate::ud::UItem;

/// Items which will be included in remote rep stats.
#[derive(Copy, Clone)]
pub struct StatRemoteRepItemKinds {
    pub module: bool,
    pub minion: bool,
}
impl StatRemoteRepItemKinds {
    /// Include all item types in remote rep stats.
    pub fn all_enabled() -> Self {
        Self {
            module: true,
            minion: true,
        }
    }
    /// Exclude all item types from remote rep stats.
    pub fn all_disabled() -> Self {
        Self {
            module: false,
            minion: false,
        }
    }
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem) -> bool {
        match u_item {
            UItem::Drone(_) => self.minion,
            UItem::Fighter(_) => self.minion,
            // Just consider everything else as modules
            _ => self.module,
        }
    }
}
