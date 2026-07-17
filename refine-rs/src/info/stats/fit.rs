use rc::UnitInterval;

use crate::{
    PValue, TriStateField,
    stats::{
        StatDmg, StatEhp, StatErps, StatHp, StatMining, StatOutReps, StatResists, StatResource, StatRps, StatSlot,
    },
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
    pub high_slots: Option<Vec<StatSlot>> = None,
    pub mid_slots: Option<Vec<StatSlot>> = None,
    pub low_slots: Option<Vec<StatSlot>> = None,
    pub turret_slots: Option<Vec<StatSlot>> = None,
    pub launcher_slots: Option<Vec<StatSlot>> = None,
    pub rig_slots: Option<Vec<StatSlot>> = None,
    pub service_slots: Option<Vec<StatSlot>> = None,
    pub subsystem_slots: Option<Vec<StatSlot>> = None,
    pub launched_drones: Option<Vec<StatSlot>> = None,
    pub launched_fighters: Option<Vec<StatSlot>> = None,
    pub launched_light_fighters: Option<Vec<StatSlot>> = None,
    pub launched_heavy_fighters: Option<Vec<StatSlot>> = None,
    pub launched_support_fighters: Option<Vec<StatSlot>> = None,
    pub launched_st_light_fighters: Option<Vec<StatSlot>> = None,
    pub launched_st_heavy_fighters: Option<Vec<StatSlot>> = None,
    pub launched_st_support_fighters: Option<Vec<StatSlot>> = None,
    // Ship tank
    pub resists: TriStateField<Vec<StatResists>> = TriStateField::Absent,
    pub hp: TriStateField<Vec<StatHp>> = TriStateField::Absent,
    pub ehp: TriStateField<Vec<StatEhp>> = TriStateField::Absent,
    pub wc_ehp: TriStateField<Vec<StatEhp>> = TriStateField::Absent,
    pub rps: TriStateField<Vec<StatRps>> = TriStateField::Absent,
    pub erps: TriStateField<Vec<StatErps>> = TriStateField::Absent,
    pub breach_resist: TriStateField<Vec<UnitInterval>> = TriStateField::Absent,
}
