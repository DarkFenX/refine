use crate::{
    ad,
    defs::{AttrVal, EItemGrpId, EItemId, SlotIndex, SolFitId, SolItemId},
    err::basic::FitFoundError,
    sol::svc::vast::{
        SolCapitalModItemInfo, SolChargeGroupValFail, SolChargeSizeValFail, SolChargeVolumeValFail,
        SolDroneGroupItemInfo, SolItemKindValFail, SolModuleStateValFail, SolValCache, SolVastSkillReq,
    },
    util::{StMap, StMapSetL1, StSet},
};

// Vast stands for VAlidation and STats.
#[derive(Clone)]
pub(in crate::sol) struct SolVast {
    pub(in crate::sol::svc::vast) fit_datas: StMap<SolFitId, SolVastFitData>,
}
impl SolVast {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            fit_datas: StMap::new(),
        }
    }
    pub(in crate::sol::svc::vast) fn get_fit_data(&self, fit_id: &SolFitId) -> Result<&SolVastFitData, FitFoundError> {
        self.fit_datas.get(fit_id).ok_or_else(|| FitFoundError::new(*fit_id))
    }
    pub(in crate::sol::svc::vast) fn get_fit_data_mut(
        &mut self,
        fit_id: &SolFitId,
    ) -> Result<&mut SolVastFitData, FitFoundError> {
        self.fit_datas
            .get_mut(fit_id)
            .ok_or_else(|| FitFoundError::new(*fit_id))
    }
}

// TODO: check if some of data containers can be merged to save time and memory (e.g. drone
// bandwidth, active drone count)
#[derive(Clone)]
pub(in crate::sol::svc::vast) struct SolVastFitData {
    // Modules with "online" effect active
    pub(in crate::sol::svc::vast) mods_online: StSet<SolItemId>,
    // Rigs with "rigSlot" effect active, with calibration cost values
    pub(in crate::sol::svc::vast) rigs_rigslot_calibration: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) drones_volume: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) drones_online_bandwidth: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) support_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) light_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) heavy_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_support_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_light_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters_online: StSet<SolItemId>,
    // Modules with "turretFitted" effect active
    pub(in crate::sol::svc::vast) mods_turret: StSet<SolItemId>,
    // Modules with "launcherFitted" effect active
    pub(in crate::sol::svc::vast) mods_launcher: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) slotted_implants: StMapSetL1<SlotIndex, SolItemId>,
    pub(in crate::sol::svc::vast) slotted_boosters: StMapSetL1<SlotIndex, SolItemId>,
    pub(in crate::sol::svc::vast) slotted_subsystems: StMapSetL1<SlotIndex, SolItemId>,
    pub(in crate::sol::svc::vast) ship_limited_mods_rigs_subs: StMap<SolItemId, ad::AItemShipLimit>,
    pub(in crate::sol::svc::vast) mods_rigs_max_group_fitted_all: StMapSetL1<EItemGrpId, SolItemId>,
    pub(in crate::sol::svc::vast) mods_rigs_max_group_fitted_limited: StMap<SolItemId, EItemGrpId>,
    pub(in crate::sol::svc::vast) mods_max_group_online_all: StMapSetL1<EItemGrpId, SolItemId>,
    pub(in crate::sol::svc::vast) mods_max_group_online_limited: StMap<SolItemId, EItemGrpId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_all: StMapSetL1<EItemGrpId, SolItemId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_limited: StMap<SolItemId, EItemGrpId>,
    pub(in crate::sol::svc::vast) rigs_rig_size: StMap<SolItemId, Option<AttrVal>>,
    pub(in crate::sol::svc::vast) srqs_skill_item_map: StMapSetL1<EItemId, SolItemId>,
    pub(in crate::sol::svc::vast) srqs_missing: StMap<SolItemId, StMap<EItemId, SolVastSkillReq>>,
    pub(in crate::sol::svc::vast) mods_charge_group: StMap<SolItemId, SolValCache<(), SolChargeGroupValFail>>,
    pub(in crate::sol::svc::vast) mods_charge_size: StMap<SolItemId, SolValCache<AttrVal, SolChargeSizeValFail>>,
    pub(in crate::sol::svc::vast) mods_charge_volume: StMap<SolItemId, SolValCache<AttrVal, SolChargeVolumeValFail>>,
    pub(in crate::sol::svc::vast) mods_capital: StMap<SolItemId, SolCapitalModItemInfo>,
    pub(in crate::sol::svc::vast) not_loaded: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) mods_state: StMap<SolItemId, SolModuleStateValFail>,
    pub(in crate::sol::svc::vast) item_kind: StMap<SolItemId, SolItemKindValFail>,
    pub(in crate::sol::svc::vast) drone_group_limit: Vec<EItemGrpId>,
    pub(in crate::sol::svc::vast) drone_group_mismatches: StMap<SolItemId, SolDroneGroupItemInfo>,
}
impl SolVastFitData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            mods_online: StSet::new(),
            rigs_rigslot_calibration: StMap::new(),
            drones_volume: StMap::new(),
            drones_online_bandwidth: StMap::new(),
            fighters_online: StSet::new(),
            support_fighters_online: StSet::new(),
            light_fighters_online: StSet::new(),
            heavy_fighters_online: StSet::new(),
            standup_support_fighters_online: StSet::new(),
            standup_light_fighters_online: StSet::new(),
            standup_heavy_fighters_online: StSet::new(),
            mods_turret: StSet::new(),
            mods_launcher: StSet::new(),
            slotted_implants: StMapSetL1::new(),
            slotted_boosters: StMapSetL1::new(),
            slotted_subsystems: StMapSetL1::new(),
            ship_limited_mods_rigs_subs: StMap::new(),
            mods_rigs_max_group_fitted_all: StMapSetL1::new(),
            mods_rigs_max_group_fitted_limited: StMap::new(),
            mods_max_group_online_all: StMapSetL1::new(),
            mods_max_group_online_limited: StMap::new(),
            mods_max_group_active_all: StMapSetL1::new(),
            mods_max_group_active_limited: StMap::new(),
            rigs_rig_size: StMap::new(),
            srqs_skill_item_map: StMapSetL1::new(),
            srqs_missing: StMap::new(),
            mods_charge_group: StMap::new(),
            mods_charge_size: StMap::new(),
            mods_charge_volume: StMap::new(),
            mods_capital: StMap::new(),
            not_loaded: StSet::new(),
            mods_state: StMap::new(),
            item_kind: StMap::new(),
            drone_group_limit: Vec::new(),
            drone_group_mismatches: StMap::new(),
        }
    }
}
