use crate::{
    info::stats::details::{
        HStatCapSim, HStatDmg, HStatJamApplied, HStatLayerEhp, HStatLayerErps, HStatLayerErpsRegen, HStatLayerHp,
        HStatLayerResist, HStatLayerRps, HStatLayerRpsRegen, HStatMining, HStatSensors, HStatTank, HStatTankRegen,
    },
    util::TriStateField,
};

#[derive(serde::Serialize)]
pub(crate) struct HItemStats {
    // Output
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) dps: TriStateField<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) volley: TriStateField<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) mps: TriStateField<Vec<HStatMining>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) outgoing_nps: TriStateField<Vec<Option<rc::AttrVal>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) outgoing_rps: TriStateField<Vec<HStatTank<rc::AttrVal>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) outgoing_cps: TriStateField<Vec<rc::AttrVal>>,
    // Tank
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
    // Cap
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_amount: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_balance: TriStateField<Vec<rc::AttrVal>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_sim: TriStateField<Vec<HStatCapSim>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) neut_resist: TriStateField<rc::AttrVal>,
    // Sensors
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
    // Mobility
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
    // Misc
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) drone_control_range: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_warp: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_gate_jump: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_drive_jump: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_dock: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_tether: TriStateField<bool>,
}
impl HItemStats {
    pub(crate) fn new() -> Self {
        Self {
            // Output
            dps: TriStateField::default(),
            volley: TriStateField::default(),
            mps: TriStateField::default(),
            outgoing_nps: TriStateField::default(),
            outgoing_rps: TriStateField::default(),
            outgoing_cps: TriStateField::default(),
            // Tank
            resists: TriStateField::default(),
            hp: TriStateField::default(),
            ehp: TriStateField::default(),
            wc_ehp: TriStateField::default(),
            rps: TriStateField::default(),
            erps: TriStateField::default(),
            // Cap
            cap_amount: TriStateField::default(),
            cap_balance: TriStateField::default(),
            cap_sim: TriStateField::default(),
            neut_resist: TriStateField::default(),
            // Sensors
            locks: TriStateField::default(),
            lock_range: TriStateField::default(),
            scan_res: TriStateField::default(),
            sensors: TriStateField::default(),
            dscan_range: TriStateField::default(),
            probing_size: TriStateField::default(),
            incoming_jam: TriStateField::default(),
            // Mobility
            speed: TriStateField::default(),
            agility: TriStateField::default(),
            align_time: TriStateField::default(),
            sig_radius: TriStateField::default(),
            mass: TriStateField::default(),
            warp_speed: TriStateField::default(),
            max_warp_range: TriStateField::default(),
            // Misc
            drone_control_range: TriStateField::default(),
            can_warp: TriStateField::default(),
            can_gate_jump: TriStateField::default(),
            can_drive_jump: TriStateField::default(),
            can_dock: TriStateField::default(),
            can_tether: TriStateField::default(),
        }
    }
}
