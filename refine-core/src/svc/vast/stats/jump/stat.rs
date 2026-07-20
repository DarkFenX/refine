use crate::{
    api::ItemTypeId,
    num::{Count, PValue},
    ud::{FitId, ItemId},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct StatJump {
    pub max_range: PValue,
    pub fuel_type_id: ItemTypeId,
    #[cfg_attr(feature = "serde", serde(rename = "self", skip_serializing_if = "Option::is_none"))]
    pub jump_self: Option<StatJumpSelf>,
    #[cfg_attr(feature = "serde", serde(rename = "conduit", skip_serializing_if = "Option::is_none"))]
    pub jump_conduit: Option<StatJumpConduit>,
    #[cfg_attr(feature = "serde", serde(rename = "portals", skip_serializing_if = "Vec::is_empty"))]
    pub jump_portals: Vec<StatJumpPortal>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct StatJumpSelf {
    pub fuel_use: Count,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct StatJumpConduit {
    pub max_passengers: Count,
    pub fuel_use_self: Count,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fuel_use_passengers: Vec<StatJumpPassenger>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct StatJumpPortal {
    pub item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fuel_use_passengers: Vec<StatJumpPassenger>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct StatJumpPassenger {
    pub fit_id: FitId,
    pub fuel_use: Option<Count>,
}
