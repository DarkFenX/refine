use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HStatJump {
    max_range: f64,
    fuel_type_id: i32,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    jump_self: Option<HStatJumpSelf>,
    #[serde(rename = "conduit", skip_serializing_if = "Option::is_none")]
    jump_conduit: Option<HStatJumpConduit>,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(rename = "bridges", skip_serializing_if = "Vec::is_empty")]
    jump_bridges: Vec<(rc::ItemId, HStatJumpBridge)>,
}

#[derive(Serialize)]
struct HStatJumpSelf {
    fuel_use: u32,
}

#[serde_as]
#[derive(Serialize)]
struct HStatJumpConduit {
    max_passengers: u32,
    fuel_use_self: u32,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    fuel_use_passengers: Vec<(rc::FitId, Option<u32>)>,
}

#[serde_as]
#[derive(Serialize)]
struct HStatJumpBridge {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    fuel_use_passengers: Vec<(rc::FitId, Option<u32>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatJump {
    pub(crate) fn from_core(core_stat: rc::stats::StatJump) -> Self {
        Self {
            max_range: core_stat.max_range.into_f64(),
            fuel_type_id: core_stat.fuel_type_id.into_i32(),
            jump_self: core_stat.jump_self.map(HStatJumpSelf::from_core),
            jump_conduit: core_stat.jump_conduit.map(HStatJumpConduit::from_core),
            jump_bridges: core_stat
                .jump_bridges
                .into_iter()
                .map(|v| (v.item_id, HStatJumpBridge::from_core(v)))
                .collect(),
        }
    }
}

impl HStatJumpSelf {
    fn from_core(core_stat: rc::stats::StatJumpSelf) -> Self {
        Self {
            fuel_use: core_stat.fuel_use.into_u32(),
        }
    }
}

impl HStatJumpConduit {
    fn from_core(core_stat: rc::stats::StatJumpConduit) -> Self {
        Self {
            max_passengers: core_stat.max_passengers.into_u32(),
            fuel_use_self: core_stat.fuel_use_self.into_u32(),
            fuel_use_passengers: core_stat
                .fuel_use_passengers
                .into_iter()
                .map(|pass_info| (pass_info.fit_id, pass_info.fuel_use.map(|v| v.into_u32())))
                .collect(),
        }
    }
}

impl HStatJumpBridge {
    fn from_core(core_stat: rc::stats::StatJumpBridge) -> Self {
        Self {
            fuel_use_passengers: core_stat
                .fuel_use_passengers
                .into_iter()
                .map(|fit_info| (fit_info.fit_id, fit_info.fuel_use.map(|v| v.into_u32())))
                .collect(),
        }
    }
}
