use crate::{
    ad,
    err::basic::FitFoundError,
    sol::{
        AttrVal, Count, FitId, ItemId, SkillLevel,
        svc::vast::{
            ValCache, ValChargeGroupFail, ValChargeSizeFail, ValChargeVolumeFail, ValFighterSquadSizeFail,
            ValItemKindFail, ValModuleStateFail, ValShipKind, VastSkillReq,
        },
    },
    util::{HMap, HMapHMap, HMapHSet, HSet},
};

// Vast stands for VAlidation and STats.
#[derive(Clone)]
pub(in crate::sol) struct Vast {
    pub(in crate::sol::svc::vast) fit_datas: HMap<FitId, VastFitData>,
}
impl Vast {
    pub(in crate::sol::svc) fn new() -> Self {
        Self { fit_datas: HMap::new() }
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
    pub(in crate::sol::svc::vast) mods_svcs_online: HSet<ItemId>,
    pub(in crate::sol::svc::vast) rigs_offline_calibration: HMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_volume: HMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_bandwidth: HMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_online_bandwidth: HMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) fighters_volume: HMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) fighters_online: HSet<ItemId>,
    pub(in crate::sol::svc::vast) support_fighters: HSet<ItemId>,
    pub(in crate::sol::svc::vast) support_fighters_online: HSet<ItemId>,
    pub(in crate::sol::svc::vast) light_fighters: HSet<ItemId>,
    pub(in crate::sol::svc::vast) light_fighters_online: HSet<ItemId>,
    pub(in crate::sol::svc::vast) heavy_fighters: HSet<ItemId>,
    pub(in crate::sol::svc::vast) heavy_fighters_online: HSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_support_fighters: HSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_support_fighters_online: HSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_light_fighters: HSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_light_fighters_online: HSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters: HSet<ItemId>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters_online: HSet<ItemId>,
    pub(in crate::sol::svc::vast) mods_turret: HSet<ItemId>,
    pub(in crate::sol::svc::vast) mods_launcher: HSet<ItemId>,
    pub(in crate::sol::svc::vast) slotted_implants: HMapHSet<ad::ASlotIndex, ItemId>,
    pub(in crate::sol::svc::vast) slotted_boosters: HMapHSet<ad::ASlotIndex, ItemId>,
    pub(in crate::sol::svc::vast) slotted_subsystems: HMapHSet<ad::ASlotIndex, ItemId>,
    pub(in crate::sol::svc::vast) ship_limited_items: HMap<ItemId, ad::AItemShipLimit>,
    pub(in crate::sol::svc::vast) mods_svcs_rigs_max_group_fitted_all: HMapHSet<ad::AItemGrpId, ItemId>,
    pub(in crate::sol::svc::vast) mods_svcs_rigs_max_group_fitted_limited: HMap<ItemId, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) mods_svcs_max_group_online_all: HMapHSet<ad::AItemGrpId, ItemId>,
    pub(in crate::sol::svc::vast) mods_svcs_max_group_online_limited: HMap<ItemId, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_all: HMapHSet<ad::AItemGrpId, ItemId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_limited: HMap<ItemId, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) rigs_rig_size: HMap<ItemId, Option<ad::AAttrVal>>,
    pub(in crate::sol::svc::vast) srqs_skill_item_map: HMapHSet<ad::AItemId, ItemId>,
    pub(in crate::sol::svc::vast) srqs_missing: HMap<ItemId, HMap<ad::AItemId, VastSkillReq>>,
    pub(in crate::sol::svc::vast) mods_charge_group: HMap<ItemId, ValCache<(), ValChargeGroupFail>>,
    pub(in crate::sol::svc::vast) mods_charge_size: HMap<ItemId, ValCache<AttrVal, ValChargeSizeFail>>,
    pub(in crate::sol::svc::vast) mods_charge_volume: HMap<ItemId, ValCache<AttrVal, ValChargeVolumeFail>>,
    pub(in crate::sol::svc::vast) mods_capital: HMap<ItemId, AttrVal>,
    pub(in crate::sol::svc::vast) not_loaded: HSet<ItemId>,
    pub(in crate::sol::svc::vast) mods_state: HMap<ItemId, ValModuleStateFail>,
    pub(in crate::sol::svc::vast) item_kind: HMap<ItemId, ValItemKindFail>,
    pub(in crate::sol::svc::vast) drone_group_limit: Vec<ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) drone_groups: HMap<ItemId, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) fighter_squad_size: HMap<ItemId, ValFighterSquadSizeFail>,
    pub(in crate::sol::svc::vast) overload_td_lvl: HMap<ItemId, SkillLevel>,
    pub(in crate::sol::svc::vast) mods_svcs_max_type_fitted: HMapHMap<ad::AItemId, ItemId, Count>,
    pub(in crate::sol::svc::vast) sec_zone_fitted: HSet<ItemId>,
    pub(in crate::sol::svc::vast) sec_zone_fitted_wspace_banned: HSet<ItemId>,
    pub(in crate::sol::svc::vast) sec_zone_online_class: HMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) sec_zone_active: HSet<ItemId>,
    pub(in crate::sol::svc::vast) sec_zone_unonlineable_class: HMap<ItemId, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) sec_zone_unactivable: HSet<ItemId>,
    pub(in crate::sol::svc::vast) mods_active: HSet<ItemId>,
    pub(in crate::sol::svc::vast) mods_rigs_svcs_vs_ship_kind: HMap<ItemId, ValShipKind>,
}
impl VastFitData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            mods_svcs_online: HSet::new(),
            rigs_offline_calibration: HMap::new(),
            drones_volume: HMap::new(),
            drones_bandwidth: HMap::new(),
            drones_online_bandwidth: HMap::new(),
            fighters_volume: HMap::new(),
            fighters_online: HSet::new(),
            support_fighters: HSet::new(),
            support_fighters_online: HSet::new(),
            light_fighters: HSet::new(),
            light_fighters_online: HSet::new(),
            heavy_fighters: HSet::new(),
            heavy_fighters_online: HSet::new(),
            standup_support_fighters: HSet::new(),
            standup_support_fighters_online: HSet::new(),
            standup_light_fighters: HSet::new(),
            standup_light_fighters_online: HSet::new(),
            standup_heavy_fighters: HSet::new(),
            standup_heavy_fighters_online: HSet::new(),
            mods_turret: HSet::new(),
            mods_launcher: HSet::new(),
            slotted_implants: HMapHSet::new(),
            slotted_boosters: HMapHSet::new(),
            slotted_subsystems: HMapHSet::new(),
            ship_limited_items: HMap::new(),
            mods_svcs_rigs_max_group_fitted_all: HMapHSet::new(),
            mods_svcs_rigs_max_group_fitted_limited: HMap::new(),
            mods_svcs_max_group_online_all: HMapHSet::new(),
            mods_svcs_max_group_online_limited: HMap::new(),
            mods_max_group_active_all: HMapHSet::new(),
            mods_max_group_active_limited: HMap::new(),
            rigs_rig_size: HMap::new(),
            srqs_skill_item_map: HMapHSet::new(),
            srqs_missing: HMap::new(),
            mods_charge_group: HMap::new(),
            mods_charge_size: HMap::new(),
            mods_charge_volume: HMap::new(),
            mods_capital: HMap::new(),
            not_loaded: HSet::new(),
            mods_state: HMap::new(),
            item_kind: HMap::new(),
            drone_group_limit: Vec::new(),
            drone_groups: HMap::new(),
            fighter_squad_size: HMap::new(),
            overload_td_lvl: HMap::new(),
            mods_svcs_max_type_fitted: HMapHMap::new(),
            sec_zone_fitted: HSet::new(),
            sec_zone_fitted_wspace_banned: HSet::new(),
            sec_zone_online_class: HMap::new(),
            sec_zone_active: HSet::new(),
            sec_zone_unonlineable_class: HMap::new(),
            sec_zone_unactivable: HSet::new(),
            mods_active: HSet::new(),
            mods_rigs_svcs_vs_ship_kind: HMap::new(),
        }
    }
}
