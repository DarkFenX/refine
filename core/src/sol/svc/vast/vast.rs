use crate::{
    ad,
    sol::{
        AttrVal, Count, FitKey, ItemKey,
        svc::vast::{
            ValCache, ValChargeGroupFailCache, ValChargeSizeFailCache, ValChargeVolumeFailCache,
            ValFighterSquadSizeFighterInfo, ValItemKindItemInfo, ValModuleStateModuleInfo, ValShipKind,
            ValSrqSkillInfo,
        },
    },
    util::{RMap, RMapRMap, RMapRSet, RSet},
};

// Vast stands for VAlidation and STats.
#[derive(Clone)]
pub(in crate::sol) struct Vast {
    pub(in crate::sol::svc::vast) fit_datas: RMap<FitKey, VastFitData>,
}
impl Vast {
    pub(in crate::sol::svc) fn new() -> Self {
        Self { fit_datas: RMap::new() }
    }
    pub(in crate::sol::svc::vast) fn get_fit_data_mut(&mut self, fit_key: &FitKey) -> &mut VastFitData {
        self.fit_datas.get_mut(fit_key).unwrap()
    }
}

// TODO: check if some of data containers can be merged to save time and memory (e.g. drone
// bandwidth, active drone count)
#[derive(Clone)]
pub(in crate::sol::svc::vast) struct VastFitData {
    pub(in crate::sol::svc::vast) mods_svcs_online: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) rigs_offline_calibration: RMap<ItemKey, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_volume: RMap<ItemKey, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_bandwidth: RMap<ItemKey, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) drones_online_bandwidth: RMap<ItemKey, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) fighters_volume: RMap<ItemKey, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) fighters_online: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) support_fighters: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) support_fighters_online: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) light_fighters: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) light_fighters_online: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) heavy_fighters: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) heavy_fighters_online: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) standup_support_fighters: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) standup_support_fighters_online: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) standup_light_fighters: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) standup_light_fighters_online: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) standup_heavy_fighters_online: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) mods_turret: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) mods_launcher: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) slotted_implants: RMapRSet<ad::ASlotIndex, ItemKey>,
    pub(in crate::sol::svc::vast) slotted_boosters: RMapRSet<ad::ASlotIndex, ItemKey>,
    pub(in crate::sol::svc::vast) slotted_subsystems: RMapRSet<ad::ASlotIndex, ItemKey>,
    pub(in crate::sol::svc::vast) ship_limited_items: RMap<ItemKey, ad::AItemShipLimit>,
    pub(in crate::sol::svc::vast) mods_svcs_rigs_max_group_fitted_all: RMapRSet<ad::AItemGrpId, ItemKey>,
    pub(in crate::sol::svc::vast) mods_svcs_rigs_max_group_fitted_limited: RMap<ItemKey, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) mods_svcs_max_group_online_all: RMapRSet<ad::AItemGrpId, ItemKey>,
    pub(in crate::sol::svc::vast) mods_svcs_max_group_online_limited: RMap<ItemKey, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) mods_max_group_active_all: RMapRSet<ad::AItemGrpId, ItemKey>,
    pub(in crate::sol::svc::vast) mods_max_group_active_limited: RMap<ItemKey, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) rigs_rig_size: RMap<ItemKey, Option<ad::AAttrVal>>,
    pub(in crate::sol::svc::vast) srqs_skill_item_map: RMapRSet<ad::AItemId, ItemKey>,
    pub(in crate::sol::svc::vast) srqs_missing: RMap<ItemKey, RMap<ad::AItemId, ValSrqSkillInfo>>,
    pub(in crate::sol::svc::vast) mods_charge_group: RMap<ItemKey, ValCache<(), ValChargeGroupFailCache>>,
    pub(in crate::sol::svc::vast) mods_charge_size: RMap<ItemKey, ValCache<AttrVal, ValChargeSizeFailCache>>,
    pub(in crate::sol::svc::vast) mods_charge_volume: RMap<ItemKey, ValCache<AttrVal, ValChargeVolumeFailCache>>,
    pub(in crate::sol::svc::vast) mods_capital: RMap<ItemKey, AttrVal>,
    pub(in crate::sol::svc::vast) not_loaded: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) mods_state: RMap<ItemKey, ValModuleStateModuleInfo>,
    pub(in crate::sol::svc::vast) item_kind: RMap<ItemKey, ValItemKindItemInfo>,
    pub(in crate::sol::svc::vast) drone_group_limit: Vec<ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) drone_groups: RMap<ItemKey, ad::AItemGrpId>,
    pub(in crate::sol::svc::vast) fighter_squad_size: RMap<ItemKey, ValFighterSquadSizeFighterInfo>,
    pub(in crate::sol::svc::vast) overload_td_lvl: RMap<ItemKey, ad::ASkillLevel>,
    pub(in crate::sol::svc::vast) mods_svcs_max_type_fitted: RMapRMap<ad::AItemId, ItemKey, Count>,
    pub(in crate::sol::svc::vast) sec_zone_fitted: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) sec_zone_fitted_wspace_banned: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) sec_zone_online_class: RMap<ItemKey, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) sec_zone_active: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) sec_zone_unonlineable_class: RMap<ItemKey, ad::AAttrVal>,
    pub(in crate::sol::svc::vast) sec_zone_unactivable: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) mods_active: RSet<ItemKey>,
    pub(in crate::sol::svc::vast) mods_rigs_svcs_vs_ship_kind: RMap<ItemKey, ValShipKind>,
}
impl VastFitData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            mods_svcs_online: RSet::new(),
            rigs_offline_calibration: RMap::new(),
            drones_volume: RMap::new(),
            drones_bandwidth: RMap::new(),
            drones_online_bandwidth: RMap::new(),
            fighters_volume: RMap::new(),
            fighters_online: RSet::new(),
            support_fighters: RSet::new(),
            support_fighters_online: RSet::new(),
            light_fighters: RSet::new(),
            light_fighters_online: RSet::new(),
            heavy_fighters: RSet::new(),
            heavy_fighters_online: RSet::new(),
            standup_support_fighters: RSet::new(),
            standup_support_fighters_online: RSet::new(),
            standup_light_fighters: RSet::new(),
            standup_light_fighters_online: RSet::new(),
            standup_heavy_fighters: RSet::new(),
            standup_heavy_fighters_online: RSet::new(),
            mods_turret: RSet::new(),
            mods_launcher: RSet::new(),
            slotted_implants: RMapRSet::new(),
            slotted_boosters: RMapRSet::new(),
            slotted_subsystems: RMapRSet::new(),
            ship_limited_items: RMap::new(),
            mods_svcs_rigs_max_group_fitted_all: RMapRSet::new(),
            mods_svcs_rigs_max_group_fitted_limited: RMap::new(),
            mods_svcs_max_group_online_all: RMapRSet::new(),
            mods_svcs_max_group_online_limited: RMap::new(),
            mods_max_group_active_all: RMapRSet::new(),
            mods_max_group_active_limited: RMap::new(),
            rigs_rig_size: RMap::new(),
            srqs_skill_item_map: RMapRSet::new(),
            srqs_missing: RMap::new(),
            mods_charge_group: RMap::new(),
            mods_charge_size: RMap::new(),
            mods_charge_volume: RMap::new(),
            mods_capital: RMap::new(),
            not_loaded: RSet::new(),
            mods_state: RMap::new(),
            item_kind: RMap::new(),
            drone_group_limit: Vec::new(),
            drone_groups: RMap::new(),
            fighter_squad_size: RMap::new(),
            overload_td_lvl: RMap::new(),
            mods_svcs_max_type_fitted: RMapRMap::new(),
            sec_zone_fitted: RSet::new(),
            sec_zone_fitted_wspace_banned: RSet::new(),
            sec_zone_online_class: RMap::new(),
            sec_zone_active: RSet::new(),
            sec_zone_unonlineable_class: RMap::new(),
            sec_zone_unactivable: RSet::new(),
            mods_active: RSet::new(),
            mods_rigs_svcs_vs_ship_kind: RMap::new(),
        }
    }
}
