use crate::stats::{StatItemStateOptions, StatMiningItemKinds, StatMiningResourceKind, StatTimeOptions};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitMining {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) item_kinds: StatMiningItemKinds = StatMiningItemKinds::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) resource_kind: StatMiningResourceKind = StatMiningResourceKind::default(),
}
const impl Default for StatOptionFitMining {
    fn default() -> Self {
        Self { .. }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionItemMining {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) resource_kind: StatMiningResourceKind = StatMiningResourceKind::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) state: StatItemStateOptions = StatItemStateOptions::default(),
}
const impl Default for StatOptionItemMining {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionFitMining {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_item_kinds(mut self, item_kinds: StatMiningItemKinds) -> Self {
        self.item_kinds = item_kinds;
        self
    }
    pub const fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
        self
    }
    pub const fn with_resource_kind(mut self, resource_kind: StatMiningResourceKind) -> Self {
        self.resource_kind = resource_kind;
        self
    }
}

impl StatOptionItemMining {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
        self
    }
    pub const fn with_resource_kind(mut self, resource_kind: StatMiningResourceKind) -> Self {
        self.resource_kind = resource_kind;
        self
    }
    pub const fn with_state(mut self, state: StatItemStateOptions) -> Self {
        self.state = state;
        self
    }
}
