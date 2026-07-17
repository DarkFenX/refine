use crate::{
    ItemId,
    stats::{StatDmgItemKinds, StatTimeOptions},
};

#[derive(Copy, Clone, Default)]
pub struct StatOptionFitDmg {
    pub item_kinds: StatDmgItemKinds = StatDmgItemKinds { default: true, .. },
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}

#[derive(Copy, Clone, Default)]
pub struct StatOptionItemDmg {
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub include_charges: bool = false,
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<ItemId> = None,
}
