use crate::stats::StatTimeOptions;

#[derive(Copy, Clone, Default)]
pub struct StatOptionFitOutCps {
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<rc::ItemId> = None,
}

#[derive(Copy, Clone, Default)]
pub struct StatOptionItemOutCps {
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<rc::ItemId> = None,
}
