use crate::{
    ItemId,
    stats::{StatNeutItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionFitOutNps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatNeutItemKinds = StatNeutItemKinds { .. },
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionItemOutNps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub include_charges: bool = false,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<ItemId> = None,
}
