use std::collections::HashMap;

use crate::{
    api::ItemTypeId,
    num::{Count, PValue},
    ud::FitId,
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
    pub fuel_use_fit: HashMap<FitId, Option<Count>>,
}

#[derive(Clone)]
pub struct StatJumpBridge {
    pub fuel_use_fit: HashMap<FitId, Option<Count>>,
}
