use crate::{
    ItemId,
    stats::{FitStats, ItemStats},
};

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct FitStatsResp {
    pub fit: FitStats,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub items: Vec<(ItemId, ItemStats)>,
}
