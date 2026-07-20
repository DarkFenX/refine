use crate::{
    ItemId,
    stats::{StatDmgItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionFitDmg {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatDmgItemKinds = StatDmgItemKinds { .. },
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionItemDmg {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub include_charges: bool = false,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<ItemId> = None,
}
