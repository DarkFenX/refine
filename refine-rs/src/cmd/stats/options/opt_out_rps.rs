use crate::{
    ItemId,
    stats::{StatOutRepItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionFitOutRps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatOutRepItemKinds = StatOutRepItemKinds { .. },
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionItemOutRps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<ItemId> = None,
}
