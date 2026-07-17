use crate::{misc::DefOption, ud::UItem};

/// Items which will be included in mining stats.
#[derive(Copy, Clone)]
pub struct StatMiningItemKinds {
    pub default: bool = true,
    pub module: DefOption = DefOption::Default,
    pub minion: DefOption = DefOption::Default,
}
impl StatMiningItemKinds {
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem) -> bool {
        match u_item {
            UItem::Drone(_) => self.minion.is_enabled(self.default),
            UItem::Module(_) => self.module.is_enabled(self.default),
            _ => false,
        }
    }
}
