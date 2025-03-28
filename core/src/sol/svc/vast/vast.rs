use crate::{
    ad,
    err::basic::FitFoundError,
    sol::{
        AttrVal, Count, FitId, ItemId, SkillLevel,
        svc::vast::{
            ValCache, ValChargeGroupFail, ValChargeSizeFail, ValChargeVolumeFail, ValFighterSquadSizeFail,
            ValItemKindFail, ValModuleStateFail, VastSkillReq,
        },
    },
    util::{StMap, StMapMap, StMapSetL1, StSet},
};

// Vast stands for VAlidation and STats.
#[derive(Clone)]
pub(in crate::sol) struct Vast {
    pub(in crate::sol::svc::vast) fit_datas: StMap<FitId, VastFitData>,
}
impl Vast {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            fit_datas: StMap::new(),
        }
    }
    pub(in crate::sol::svc::vast) fn get_fit_data(&self, fit_id: &FitId) -> Result<&VastFitData, FitFoundError> {
        self.fit_datas.get(fit_id).ok_or(FitFoundError { fit_id: *fit_id })
    }
    pub(in crate::sol::svc::vast) fn get_fit_data_mut(
        &mut self,
        fit_id: &FitId,
    ) -> Result<&mut VastFitData, FitFoundError> {
        self.fit_datas.get_mut(fit_id).ok_or(FitFoundError { fit_id: *fit_id })
    }
}

// TODO: check if some of data containers can be merged to save time and memory (e.g. drone
// bandwidth, active drone count)
#[derive(Clone)]
pub(in crate::sol::svc::vast) struct VastFitData {
    // Modules with "online" effect active
    pub(in crate::sol::svc::vast) mods_svcs_online: StSet<ItemId>,
    // Rigs with "rigSlot" effect active, with calibration cost values
    pub(in crate::sol::svc::vast) rigs_rigslot_calibration: StMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_volume: StMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_bandwidth: StMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_online_bandwidth: StMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) fighters_volume: StMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) fighters_online: StSet<ItemId>,
    pub(in crate::sol::svc::vast) support_fighters: StSet<ItemId>,
    pub(in crate::sol::svc::vast) support_fighters_online: StSet<ItemId>,
    pub(in crate::sol::svc::vast) light_fighters: StSet<ItemId>,
    pub(in crate::sol::svc::vast) light_fighters_online: StSet<ItemId>,
    pub(in crate::sol::svc::vast) heavy_fighters: StSet<ItemId>,
    pub(in crate::sol::svc::vast) heavy_fighters_online: StSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_support_fighters: StSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_support_fighters_online: StSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_light_fighters: StSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_light_fighters_online: StSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters: StSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters_online: StSet<ItemId>,
    // Modules with "turretFitted" effect active
    pub(in crate::sol::svc::vast) mods_turret: StSet<ItemId>,
    // Modules with "launcherFitted" effect active
    pub(in crate::sol::svc::vast) mods_launcher: StSet<ItemId>,
    pub(in crate::sol::svc::vast) slotted_implants: StMapSetL1<ad::ASlotIndex, ItemId>,
    pub(in crate::sol::svc::vast) slotted_boosters: StMapSetL1<ad::ASlotIndex, ItemId>,
    pub(in crate::sol::svc::vast) slotted_subsystems: StMapSetL1<ad::ASlotIndex, ItemId>,
    pub(in crate::sol::svc::vast) ship_limited_items: StMap<ItemId, ad::AItemShipLimit>,
    pub(in crate::sol::svc::vast) mods_svcs_rigs_max_group_fitted_all: StMapSetL1<ad::AItemGrpId, ItemId>,
    pub(in crate::sol::svc::vast) mods_svcs_rigs_max_group_fitted_limited: StMap<ItemId, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) mods_svcs_max_group_online_all: StMapSetL1<ad::AItemGrpId, ItemId>,
    pub(in crate::sol::svc::vast) mods_svcs_max_group_online_limited: StMap<ItemId, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_all: StMapSetL1<ad::AItemGrpId, ItemId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_limited: StMap<ItemId, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) rigs_rig_size: StMap<ItemId, Option<ad::AAttrVal>>,
    pub(in crate::sol::svc::vast) srqs_skill_item_map: StMapSetL1<ad::AItemId, ItemId>,
    pub(in crate::sol::svc::vast) srqs_missing: StMap<ItemId, StMap<ad::AItemId, VastSkillReq>>,
    pub(in crate::sol::svc::vast) mods_charge_group: StMap<ItemId, ValCache<(), ValChargeGroupFail>>,
    pub(in crate::sol::svc::vast) mods_charge_size: StMap<ItemId, ValCache<AttrVal, ValChargeSizeFail>>,
    pub(in crate::sol::svc::vast) mods_charge_volume: StMap<ItemId, ValCache<AttrVal, ValChargeVolumeFail>>,
    pub(in crate::sol::svc::vast) mods_capital: StMap<ItemId, AttrVal>,
    pub(in crate::sol::svc::vast) not_loaded: StSet<ItemId>,
    pub(in crate::sol::svc::vast) mods_state: StMap<ItemId, ValModuleStateFail>,
    pub(in crate::sol::svc::vast) item_kind: StMap<ItemId, ValItemKindFail>,
    pub(in crate::sol::svc::vast) drone_group_limit: Vec<ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) drone_groups: StMap<ItemId, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) fighter_squad_size: StMap<ItemId, ValFighterSquadSizeFail>,
    pub(in crate::sol::svc::vast) overload_td_lvl: StMap<ItemId, SkillLevel>,
    pub(in crate::sol::svc::vast) mods_svcs_max_type_fitted: StMapMap<ad::AItemId, ItemId, Count>,
    pub(in crate::sol::svc::vast) sec_zone_fitted: StSet<ItemId>,
    pub(in crate::sol::svc::vast) sec_zone_fitted_wspace_banned: StSet<ItemId>,
    pub(in crate::sol::svc::vast) sec_zone_online_class: StMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) sec_zone_active: StSet<ItemId>,
    pub(in crate::sol::svc::vast) sec_zone_unonlineable_class: StMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) sec_zone_unactivable: StSet<ItemId>,
    pub(in crate::sol::svc::vast) mods_active: StSet<ItemId>,
}
impl VastFitData {
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
            fighter_squad_size: StMap::new(),
            overload_td_lvl: StMap::new(),
            mods_svcs_max_type_fitted: StMapMap::new(),
            sec_zone_fitted: StSet::new(),
            sec_zone_fitted_wspace_banned: StSet::new(),
            sec_zone_online_class: StMap::new(),
            sec_zone_active: StSet::new(),
            sec_zone_unonlineable_class: StMap::new(),
            sec_zone_unactivable: StSet::new(),
            mods_active: StSet::new(),
        }
    }
}
