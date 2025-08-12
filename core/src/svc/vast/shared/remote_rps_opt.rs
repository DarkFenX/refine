use crate::ud::UItem;

/// Items which will be included in remote rep stats.
#[derive(Copy, Clone)]
pub struct StatRemoteRpsItemKinds {
    pub modules: bool,
    pub minions: bool,
}
impl StatRemoteRpsItemKinds {
    /// Include all item types in remote rep stats.
    pub fn all_enabled() -> Self {
        Self {
            modules: true,
            minions: true,
        }
    }
    /// Exclude all item types from remote rep stats.
    pub fn all_disabled() -> Self {
        Self {
            modules: false,
            minions: false,
        }
    }
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem) -> bool {
        match u_item {
            UItem::Drone(_) => self.minions,
            UItem::Fighter(_) => self.minions,
            // Just consider everything else as modules for now
            _ => self.modules,
        }
    }
}
