use crate::{
    info::stats::details::{
        HStatDmg, HStatLayerEhp, HStatLayerErps, HStatLayerHp, HStatLayerResist, HStatLayerRps, HStatRes, HStatSlot,
        HStatTank,
    },
    util::TriStateField,
};

#[derive(serde::Serialize)]
pub(crate) struct HFitStats {
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
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) agility: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) align_time: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) speed: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dps: Option<Vec<HStatDmg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) volley: Option<Vec<HStatDmg>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) hp: TriStateField<HStatTank<HStatLayerHp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) ehp: TriStateField<Vec<HStatTank<Option<HStatLayerEhp>>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) wc_ehp: TriStateField<HStatTank<Option<HStatLayerEhp>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rps: TriStateField<Vec<HStatTank<HStatLayerRps>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) erps: TriStateField<Vec<HStatTank<Option<HStatLayerErps>>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) resists: TriStateField<HStatTank<HStatLayerResist>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) remote_rps: Option<Vec<HStatTank<rc::AttrVal>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) remote_cps: Option<rc::AttrVal>,
}
impl HFitStats {
    pub fn new() -> Self {
        Self {
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
            cpu: Option::default(),
            powergrid: Option::default(),
            calibration: Option::default(),
            drone_bay_volume: Option::default(),
            drone_bandwidth: Option::default(),
            fighter_bay_volume: Option::default(),
            agility: TriStateField::default(),
            align_time: TriStateField::default(),
            speed: TriStateField::default(),
            dps: Option::default(),
            volley: Option::default(),
            hp: TriStateField::default(),
            ehp: TriStateField::default(),
            wc_ehp: TriStateField::default(),
            rps: TriStateField::default(),
            erps: TriStateField::default(),
            resists: TriStateField::default(),
            remote_rps: Option::default(),
            remote_cps: Option::default(),
        }
    }
}
