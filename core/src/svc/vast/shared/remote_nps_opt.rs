use crate::ud::UItem;

/// Items which will be included in neut stats.
#[derive(Copy, Clone)]
pub struct StatRemoteNpsItemKinds {
    pub module: bool,
    pub minion: bool,
    pub bomb: bool,
}
impl StatRemoteNpsItemKinds {
    /// Include all item types in neut stats.
    pub fn all_enabled() -> Self {
        Self {
            module: true,
            minion: true,
            bomb: true,
        }
    }
    /// Exclude all item types from neut stats.
    pub fn all_disabled() -> Self {
        Self {
            module: false,
            minion: false,
            bomb: false,
        }
    }
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem) -> bool {
        match u_item {
            // Consider all charges bombs for simplicity, there are no other charges which neut
            UItem::Charge(_) => self.bomb,
            UItem::Drone(_) => self.minion,
            UItem::Fighter(_) => self.minion,
            // Just consider everything else as modules
            _ => self.module,
        }
    }
}
