use crate::{
    ItemId,
    stats::{StatItemChargeOptions, StatItemStateOptions, StatNeutItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionFitOutNps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatNeutItemKinds = StatNeutItemKinds { .. },
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionItemOutNps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub charges: StatItemChargeOptions = StatItemChargeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub state: StatItemStateOptions = StatItemStateOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}
