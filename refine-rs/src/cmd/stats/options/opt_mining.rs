use crate::stats::{StatItemStateOptions, StatMiningItemKinds, StatMiningResourceKind, StatTimeOptions};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionFitMining {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatMiningItemKinds = StatMiningItemKinds { .. },
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub resource_kind: StatMiningResourceKind = StatMiningResourceKind::default(),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionItemMining {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub resource_kind: StatMiningResourceKind = StatMiningResourceKind::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub state: StatItemStateOptions = StatItemStateOptions::default(),
}
