use crate::stats::{StatMiningItemKinds, StatTimeOptions};

#[derive(Copy, Clone, Default)]
pub struct StatOptionFitMining {
    pub item_kinds: StatMiningItemKinds = StatMiningItemKinds { default: true, .. },
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub mission: bool = false,
}

#[derive(Copy, Clone, Default)]
pub struct StatOptionItemMining {
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub mission: bool = false,
    pub ignore_state: bool = false,
}
