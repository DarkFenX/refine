use crate::{
    api::ItemTypeId,
    num::{Count, PValue},
    ud::{FitId, ItemId},
};

#[derive(Clone)]
pub struct StatJump {
    pub max_range: PValue,
    pub fuel_type_id: ItemTypeId,
    pub jump_self: Option<StatJumpSelf>,
    pub jump_conduit: Option<StatJumpConduit>,
    pub jump_bridge: Vec<StatJumpBridge>,
}

#[derive(Copy, Clone)]
pub struct StatJumpSelf {
    pub fuel_use: Count,
}

#[derive(Clone)]
pub struct StatJumpConduit {
    pub max_passengers: Count,
    pub fuel_use_self: Count,
    pub fuel_use_fit: Vec<StatJumpPassenger>,
}

#[derive(Clone)]
pub struct StatJumpBridge {
    pub item_id: ItemId,
    pub fuel_use_fit: Vec<StatJumpPassenger>,
}

#[derive(Copy, Clone)]
pub struct StatJumpPassenger {
    pub fit_id: FitId,
    pub fuel_use: Option<Count>,
}
