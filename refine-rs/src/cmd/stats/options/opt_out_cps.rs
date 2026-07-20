use crate::stats::StatTimeOptions;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionFitOutCps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<rc::ItemId> = None,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionItemOutCps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<rc::ItemId> = None,
}
