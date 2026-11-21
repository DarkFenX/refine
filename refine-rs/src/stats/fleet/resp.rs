use crate::{
    FitId, ItemId,
    stats::{FitStats, FleetStats, ItemStats},
};

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct FleetStatsResp {
    pub fleet: FleetStats,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub fits: Vec<(FitId, FitStats)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub items: Vec<(ItemId, ItemStats)>,
}
impl FleetStatsResp {
    pub fn get_fit(&self, fit_id: &FitId) -> Option<&FitStats> {
        self.fits.iter().find_map(|(id, fit_stats)| match id == fit_id {
            true => Some(fit_stats),
            false => None,
        })
    }
    pub fn get_item(&self, item_id: &ItemId) -> Option<&ItemStats> {
        self.items.iter().find_map(|(id, item_stats)| match id == item_id {
            true => Some(item_stats),
            false => None,
        })
    }
}
