use crate::{misc::DefOption, ud::UItem};

/// Items which will be included in outgoing rep stats.
#[derive(Copy, Clone, Default)]
pub struct StatOutRepItemKinds {
    pub default: bool = true,
    pub module: DefOption = DefOption::Default,
    pub minion: DefOption = DefOption::Default,
}
impl StatOutRepItemKinds {
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem) -> bool {
        match u_item {
            UItem::Drone(_) => self.minion.is_enabled(self.default),
            UItem::Fighter(_) => self.minion.is_enabled(self.default),
            // Just consider everything else as modules
            _ => self.module.is_enabled(self.default),
        }
    }
}
