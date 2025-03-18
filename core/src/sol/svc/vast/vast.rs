use crate::{
    ad,
    defs::{AttrVal, Count, EItemGrpId, EItemId, SkillLevel, SlotIndex, SolFitId, SolItemId},
    err::basic::FitFoundError,
    sol::svc::vast::{
        SolValCache, SolValChargeGroupFail, SolValChargeSizeFail, SolValChargeVolumeFail, SolValFighterCountFail,
        SolValItemKindFail, SolValModuleStateFail, SolVastSkillReq,
    },
    util::{StMap, StMapMap, StMapSetL1, StSet},
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
    pub(in crate::sol::svc::vast) mods_svcs_online: StSet<SolItemId>,
    // Rigs with "rigSlot" effect active, with calibration cost values
    pub(in crate::sol::svc::vast) rigs_rigslot_calibration: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) drones_volume: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) drones_bandwidth: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) drones_online_bandwidth: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) fighters_volume: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) support_fighters: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) support_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) light_fighters: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) light_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) heavy_fighters: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) heavy_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_support_fighters: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_support_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_light_fighters: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_light_fighters_online: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters_online: StSet<SolItemId>,
    // Modules with "turretFitted" effect active
    pub(in crate::sol::svc::vast) mods_turret: StSet<SolItemId>,
    // Modules with "launcherFitted" effect active
    pub(in crate::sol::svc::vast) mods_launcher: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) slotted_implants: StMapSetL1<SlotIndex, SolItemId>,
    pub(in crate::sol::svc::vast) slotted_boosters: StMapSetL1<SlotIndex, SolItemId>,
    pub(in crate::sol::svc::vast) slotted_subsystems: StMapSetL1<SlotIndex, SolItemId>,
    pub(in crate::sol::svc::vast) ship_limited_items: StMap<SolItemId, ad::AItemShipLimit>,
    pub(in crate::sol::svc::vast) mods_svcs_rigs_max_group_fitted_all: StMapSetL1<EItemGrpId, SolItemId>,
    pub(in crate::sol::svc::vast) mods_svcs_rigs_max_group_fitted_limited: StMap<SolItemId, EItemGrpId>,
    pub(in crate::sol::svc::vast) mods_svcs_max_group_online_all: StMapSetL1<EItemGrpId, SolItemId>,
    pub(in crate::sol::svc::vast) mods_svcs_max_group_online_limited: StMap<SolItemId, EItemGrpId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_all: StMapSetL1<EItemGrpId, SolItemId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_limited: StMap<SolItemId, EItemGrpId>,
    pub(in crate::sol::svc::vast) rigs_rig_size: StMap<SolItemId, Option<AttrVal>>,
    pub(in crate::sol::svc::vast) srqs_skill_item_map: StMapSetL1<EItemId, SolItemId>,
    pub(in crate::sol::svc::vast) srqs_missing: StMap<SolItemId, StMap<EItemId, SolVastSkillReq>>,
    pub(in crate::sol::svc::vast) mods_charge_group: StMap<SolItemId, SolValCache<(), SolValChargeGroupFail>>,
    pub(in crate::sol::svc::vast) mods_charge_size: StMap<SolItemId, SolValCache<AttrVal, SolValChargeSizeFail>>,
    pub(in crate::sol::svc::vast) mods_charge_volume: StMap<SolItemId, SolValCache<AttrVal, SolValChargeVolumeFail>>,
    pub(in crate::sol::svc::vast) mods_capital: StMap<SolItemId, AttrVal>,
    pub(in crate::sol::svc::vast) not_loaded: StSet<SolItemId>,
    pub(in crate::sol::svc::vast) mods_state: StMap<SolItemId, SolValModuleStateFail>,
    pub(in crate::sol::svc::vast) item_kind: StMap<SolItemId, SolValItemKindFail>,
    pub(in crate::sol::svc::vast) drone_group_limit: Vec<EItemGrpId>,
    pub(in crate::sol::svc::vast) drone_groups: StMap<SolItemId, EItemGrpId>,
    pub(in crate::sol::svc::vast) fighter_count: StMap<SolItemId, SolValFighterCountFail>,
    pub(in crate::sol::svc::vast) overload_td_lvl: StMap<SolItemId, SkillLevel>,
    pub(in crate::sol::svc::vast) mods_svcs_max_type_fitted: StMapMap<EItemId, SolItemId, Count>,
}
impl SolVastFitData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            mods_svcs_online: StSet::new(),
            rigs_rigslot_calibration: StMap::new(),
            drones_volume: StMap::new(),
            drones_bandwidth: StMap::new(),
            drones_online_bandwidth: StMap::new(),
            fighters_volume: StMap::new(),
            fighters_online: StSet::new(),
            support_fighters: StSet::new(),
            support_fighters_online: StSet::new(),
            light_fighters: StSet::new(),
            light_fighters_online: StSet::new(),
            heavy_fighters: StSet::new(),
            heavy_fighters_online: StSet::new(),
            standup_support_fighters: StSet::new(),
            standup_support_fighters_online: StSet::new(),
            standup_light_fighters: StSet::new(),
            standup_light_fighters_online: StSet::new(),
            standup_heavy_fighters: StSet::new(),
            standup_heavy_fighters_online: StSet::new(),
            mods_turret: StSet::new(),
            mods_launcher: StSet::new(),
            slotted_implants: StMapSetL1::new(),
            slotted_boosters: StMapSetL1::new(),
            slotted_subsystems: StMapSetL1::new(),
            ship_limited_items: StMap::new(),
            mods_svcs_rigs_max_group_fitted_all: StMapSetL1::new(),
            mods_svcs_rigs_max_group_fitted_limited: StMap::new(),
            mods_svcs_max_group_online_all: StMapSetL1::new(),
            mods_svcs_max_group_online_limited: StMap::new(),
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
            drone_groups: StMap::new(),
            fighter_count: StMap::new(),
            overload_td_lvl: StMap::new(),
            mods_svcs_max_type_fitted: StMapMap::new(),
        }
    }
}
