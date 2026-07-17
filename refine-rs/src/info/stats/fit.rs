use crate::{
    PValue,
    stats::{StatDmg, StatMining, StatOutReps, StatResource, StatSlot},
};

pub struct FitStats {
    // Fit output stats
    pub dmg: Option<Vec<Option<StatDmg>>> = None,
    pub mps: Option<Vec<StatMining>> = None,
    pub outgoing_nps: Option<Vec<Option<PValue>>> = None,
    pub outgoing_rps: Option<Vec<Option<StatOutReps>>> = None,
    pub outgoing_cps: Option<Vec<Option<PValue>>> = None,
    // Fit resources
    pub cpu: Option<Vec<StatResource>> = None,
    pub powergrid: Option<Vec<StatResource>> = None,
    pub calibration: Option<Vec<StatResource>> = None,
    pub drone_bay_volume: Option<Vec<StatResource>> = None,
    pub drone_bandwidth: Option<Vec<StatResource>> = None,
    pub fighter_bay_volume: Option<Vec<StatResource>> = None,
    // Fit slots
    pub(crate) high_slots: Option<Vec<StatSlot>> = None,
    pub(crate) mid_slots: Option<Vec<StatSlot>> = None,
    pub(crate) low_slots: Option<Vec<StatSlot>> = None,
    pub(crate) turret_slots: Option<Vec<StatSlot>> = None,
    pub(crate) launcher_slots: Option<Vec<StatSlot>> = None,
    pub(crate) rig_slots: Option<Vec<StatSlot>> = None,
    pub(crate) service_slots: Option<Vec<StatSlot>> = None,
    pub(crate) subsystem_slots: Option<Vec<StatSlot>> = None,
    pub(crate) launched_drones: Option<Vec<StatSlot>> = None,
    pub(crate) launched_fighters: Option<Vec<StatSlot>> = None,
    pub(crate) launched_light_fighters: Option<Vec<StatSlot>> = None,
    pub(crate) launched_heavy_fighters: Option<Vec<StatSlot>> = None,
    pub(crate) launched_support_fighters: Option<Vec<StatSlot>> = None,
    pub(crate) launched_st_light_fighters: Option<Vec<StatSlot>> = None,
    pub(crate) launched_st_heavy_fighters: Option<Vec<StatSlot>> = None,
    pub(crate) launched_st_support_fighters: Option<Vec<StatSlot>> = None,
}
