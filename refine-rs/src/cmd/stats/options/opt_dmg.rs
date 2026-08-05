use crate::{
    ItemId,
    stats::{StatChargeOptions, StatCritOptions, StatDmgItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionFitDmg {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatDmgItemKinds = StatDmgItemKinds { .. },
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub crits: StatCritOptions = StatCritOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionItemDmg {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub crits: StatCritOptions = StatCritOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub charges: StatChargeOptions = StatChargeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<ItemId> = None,
}
