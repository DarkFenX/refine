use crate::stats::ItemStats;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ItemStatsResp {
    pub item: ItemStats,
}
