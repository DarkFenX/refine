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
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::include_crits_default"))]
    pub include_crits: bool = true,
    pub projectee_item_id: Option<ItemId> = None,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionItemDmg {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::include_crits_default"))]
    pub include_crits: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    pub include_charges: bool = false,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<ItemId> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    pub(super) fn include_crits_default() -> bool {
        true
    }
}
