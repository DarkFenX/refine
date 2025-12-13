use crate::{
    info::stats::details::{
        HStatCapSim, HStatDmg, HStatJamApplied, HStatLayerEhp, HStatLayerErps, HStatLayerErpsRegen, HStatLayerHp,
        HStatLayerResist, HStatLayerRps, HStatLayerRpsRegen, HStatMining, HStatRes, HStatSensors, HStatSlot, HStatTank,
        HStatTankRegen,
    },
    util::TriStateField,
};

#[derive(serde::Serialize)]
pub(crate) struct HFitStats {
    // Fit output stats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dps: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) volley: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mps: Option<Vec<HStatMining>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_nps: Option<Vec<Option<rc::AttrVal>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_rps: Option<Vec<HStatTank<rc::AttrVal>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_cps: Option<rc::AttrVal>,
    // Fit resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cpu: Option<HStatRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) powergrid: Option<HStatRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) calibration: Option<HStatRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) drone_bay_volume: Option<HStatRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) drone_bandwidth: Option<HStatRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fighter_bay_volume: Option<HStatRes>,
    // Fit slots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) high_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mid_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) low_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) turret_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launcher_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rig_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) service_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) subsystem_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_drones: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_light_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_heavy_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_support_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_st_light_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_st_heavy_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_st_support_fighters: Option<HStatSlot>,
    // Ship tank
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) resists: TriStateField<HStatTank<HStatLayerResist>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) hp: TriStateField<HStatTank<HStatLayerHp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) ehp: TriStateField<Vec<HStatTank<Option<HStatLayerEhp>>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) wc_ehp: TriStateField<HStatTank<Option<HStatLayerEhp>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rps: TriStateField<Vec<HStatTankRegen<HStatLayerRps, HStatLayerRpsRegen>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) erps: TriStateField<Vec<HStatTankRegen<Option<HStatLayerErps>, Option<HStatLayerErpsRegen>>>>,
    // Ship cap
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_amount: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_balance: TriStateField<Vec<rc::AttrVal>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_sim: TriStateField<Vec<HStatCapSim>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) neut_resist: TriStateField<rc::AttrVal>,
    // Ship sensors
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) locks: TriStateField<rc::Count>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) lock_range: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) scan_res: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) sensors: TriStateField<HStatSensors>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) dscan_range: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) probing_size: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) incoming_jam: TriStateField<HStatJamApplied>,
    // Ship mobility
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) speed: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) agility: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) align_time: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) sig_radius: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) mass: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) warp_speed: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) max_warp_range: TriStateField<rc::AttrVal>,
    // Ship misc stats
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) drone_control_range: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_warp: TriStateField<bool>,
}
impl HFitStats {
    pub(crate) fn new() -> Self {
        Self {
            // Fit output stats
            dps: Option::default(),
            volley: Option::default(),
            mps: Option::default(),
            outgoing_nps: Option::default(),
            outgoing_rps: Option::default(),
            outgoing_cps: Option::default(),
            // Fit resources
            cpu: Option::default(),
            powergrid: Option::default(),
            calibration: Option::default(),
            drone_bay_volume: Option::default(),
            drone_bandwidth: Option::default(),
            fighter_bay_volume: Option::default(),
            // Fit slots
            high_slots: Option::default(),
            mid_slots: Option::default(),
            low_slots: Option::default(),
            turret_slots: Option::default(),
            launcher_slots: Option::default(),
            rig_slots: Option::default(),
            service_slots: Option::default(),
            subsystem_slots: Option::default(),
            launched_drones: Option::default(),
            launched_fighters: Option::default(),
            launched_light_fighters: Option::default(),
            launched_heavy_fighters: Option::default(),
            launched_support_fighters: Option::default(),
            launched_st_light_fighters: Option::default(),
            launched_st_heavy_fighters: Option::default(),
            launched_st_support_fighters: Option::default(),
            // Ship tank
            resists: TriStateField::default(),
            hp: TriStateField::default(),
            ehp: TriStateField::default(),
            wc_ehp: TriStateField::default(),
            rps: TriStateField::default(),
            erps: TriStateField::default(),
            // Ship cap
            cap_amount: TriStateField::default(),
            cap_balance: TriStateField::default(),
            cap_sim: TriStateField::default(),
            neut_resist: TriStateField::default(),
            // Ship sensors
            locks: TriStateField::default(),
            lock_range: TriStateField::default(),
            scan_res: TriStateField::default(),
            sensors: TriStateField::default(),
            dscan_range: TriStateField::default(),
            probing_size: TriStateField::default(),
            incoming_jam: TriStateField::default(),
            // Ship mobility
            speed: TriStateField::default(),
            agility: TriStateField::default(),
            align_time: TriStateField::default(),
            sig_radius: TriStateField::default(),
            mass: TriStateField::default(),
            warp_speed: TriStateField::default(),
            max_warp_range: TriStateField::default(),
            // Ship misc stats
            drone_control_range: TriStateField::default(),
            can_warp: TriStateField::default(),
        }
    }
}
