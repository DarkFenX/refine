use crate::{
    ad::{AItemGrpId, AItemId},
    misc::{AttrSpec, DmgKinds, Ecm, EffectSpec, MiningAmount},
    nd::NBreacherDmgGetter,
    num::{Count, PValue, SkillLevel, SlotIndex, Value},
    rd::{RAttrId, REffectId, REffectLocalOpcSpec, REffectProjOpcSpec, RItemListId, RItemShipLimit},
    svc::vast::{
        ValFighterSquadSizeFighterInfo, ValItemKindItemInfo, ValModuleStateModuleInfo, ValShipKind, ValSrqSkillInfo,
        vaste_vals::EffectSecZoneInfo,
    },
    ud::{UFitId, UItemId},
    util::{RMap, RMapRMap, RMapRMapRMap, RMapRSet, RSet},
};

// Vast stands for VAlidation and STats.
#[derive(Clone)]
pub(in crate::svc) struct Vast {
    pub(in crate::svc::vast) fit_datas: RMap<UFitId, VastFitData>,
    pub(in crate::svc::vast) not_loaded: RSet<UItemId>,
    // Incoming remote reps
    pub(in crate::svc::vast) irr_shield: RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    pub(in crate::svc::vast) irr_shield_limitable:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    pub(in crate::svc::vast) irr_armor: RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    pub(in crate::svc::vast) irr_armor_limitable: RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    pub(in crate::svc::vast) irr_hull: RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    // Cap
    pub(in crate::svc::vast) in_cap: RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    pub(in crate::svc::vast) in_neuts: RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    // Ewar
    pub(in crate::svc::vast) in_ecm: RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<Ecm>>,
}
impl Vast {
    pub(in crate::svc) fn new() -> Self {
        Self {
            fit_datas: RMap::new(),
            not_loaded: RSet::new(),
            irr_shield: RMapRMapRMap::new(),
            irr_shield_limitable: RMapRMapRMap::new(),
            irr_armor: RMapRMapRMap::new(),
            irr_armor_limitable: RMapRMapRMap::new(),
            irr_hull: RMapRMapRMap::new(),
            in_cap: RMapRMapRMap::new(),
            in_neuts: RMapRMapRMap::new(),
            in_ecm: RMapRMapRMap::new(),
        }
    }
    pub(in crate::svc) fn get_fit_data(&self, fit_uid: &UFitId) -> &VastFitData {
        self.fit_datas.get(fit_uid).unwrap()
    }
    pub(in crate::svc::vast) fn get_fit_data_mut(&mut self, fit_uid: &UFitId) -> &mut VastFitData {
        self.fit_datas.get_mut(fit_uid).unwrap()
    }
}

// TODO: check if some of data containers can be merged to save time and memory (e.g. drone
// TODO: bandwidth, active drone count), reorder validations
#[derive(Clone)]
pub(in crate::svc) struct VastFitData {
    // Validation-related
    pub(in crate::svc::vast) mods_svcs_online: RSet<UItemId>,
    pub(in crate::svc::vast) rigs_offline_calibration: RMap<UItemId, Value>,
    pub(in crate::svc::vast) drones_volume: RMap<UItemId, PValue>,
    pub(in crate::svc::vast) drones_bandwidth: RMap<UItemId, Value>,
    pub(in crate::svc::vast) drones_online_bandwidth: RMap<UItemId, Value>,
    pub(in crate::svc::vast) fighters_volume: RMap<UItemId, PValue>,
    pub(in crate::svc::vast) fighters_online: RSet<UItemId>,
    pub(in crate::svc::vast) light_fighters: RSet<UItemId>,
    pub(in crate::svc::vast) light_fighters_online: RSet<UItemId>,
    pub(in crate::svc::vast) heavy_fighters: RSet<UItemId>,
    pub(in crate::svc::vast) heavy_fighters_online: RSet<UItemId>,
    pub(in crate::svc::vast) support_fighters: RSet<UItemId>,
    pub(in crate::svc::vast) support_fighters_online: RSet<UItemId>,
    pub(in crate::svc::vast) st_light_fighters: RSet<UItemId>,
    pub(in crate::svc::vast) st_light_fighters_online: RSet<UItemId>,
    pub(in crate::svc::vast) st_heavy_fighters: RSet<UItemId>,
    pub(in crate::svc::vast) st_heavy_fighters_online: RSet<UItemId>,
    pub(in crate::svc::vast) st_support_fighters: RSet<UItemId>,
    pub(in crate::svc::vast) st_support_fighters_online: RSet<UItemId>,
    pub(in crate::svc::vast) mods_turret: RSet<UItemId>,
    pub(in crate::svc::vast) mods_launcher: RSet<UItemId>,
    pub(in crate::svc::vast) slotted_implants: RMapRSet<SlotIndex, UItemId>,
    pub(in crate::svc::vast) slotted_boosters: RMapRSet<SlotIndex, UItemId>,
    pub(in crate::svc::vast) slotted_subsystems: RMapRSet<SlotIndex, UItemId>,
    pub(in crate::svc::vast) ship_limited_items: RMap<UItemId, RItemShipLimit>,
    pub(in crate::svc::vast) mods_svcs_rigs_max_group_fitted_all: RMapRSet<AItemGrpId, UItemId>,
    pub(in crate::svc::vast) mods_svcs_rigs_max_group_fitted_limited: RMap<UItemId, AItemGrpId>,
    pub(in crate::svc::vast) mods_svcs_max_group_online_all: RMapRSet<AItemGrpId, UItemId>,
    pub(in crate::svc::vast) mods_svcs_max_group_online_limited: RMap<UItemId, AItemGrpId>,
    pub(in crate::svc::vast) mods_max_group_active_all: RMapRSet<AItemGrpId, UItemId>,
    pub(in crate::svc::vast) mods_max_group_active_limited: RMap<UItemId, AItemGrpId>,
    pub(in crate::svc::vast) rigs_rig_size: RMap<UItemId, Option<Value>>,
    pub(in crate::svc::vast) srqs_skill_item_map: RMapRSet<AItemId, UItemId>,
    pub(in crate::svc::vast) srqs_missing: RMap<UItemId, RMap<AItemId, ValSrqSkillInfo>>,
    pub(in crate::svc::vast) charge_group: RMap<UItemId, UItemId>,
    pub(in crate::svc::vast) charge_cont_group: RMap<UItemId, UItemId>,
    pub(in crate::svc::vast) charge_size: RMap<UItemId, UItemId>,
    pub(in crate::svc::vast) charge_volume: RMap<UItemId, UItemId>,
    pub(in crate::svc::vast) mods_capital: RMap<UItemId, PValue>,
    pub(in crate::svc::vast) not_loaded: RSet<UItemId>,
    pub(in crate::svc::vast) mods_state: RMap<UItemId, ValModuleStateModuleInfo>,
    pub(in crate::svc::vast) item_kind: RMap<UItemId, ValItemKindItemInfo>,
    pub(in crate::svc::vast) drone_group_limit: Vec<AItemGrpId>,
    pub(in crate::svc::vast) drone_groups: RMap<UItemId, AItemGrpId>,
    pub(in crate::svc::vast) fighter_squad_size: RMap<UItemId, ValFighterSquadSizeFighterInfo>,
    pub(in crate::svc::vast) overload_td_lvl: RMap<UItemId, SkillLevel>,
    pub(in crate::svc::vast) mods_svcs_max_type_fitted: RMapRMap<AItemId, UItemId, Count>,
    pub(in crate::svc::vast) sec_zone_fitted: RSet<UItemId>,
    pub(in crate::svc::vast) sec_zone_fitted_wspace_banned: RSet<UItemId>,
    pub(in crate::svc::vast) sec_zone_online_class: RMap<UItemId, Value>,
    pub(in crate::svc::vast) sec_zone_active: RSet<UItemId>,
    pub(in crate::svc::vast) sec_zone_unonlineable_class: RMap<UItemId, Value>,
    pub(in crate::svc::vast) sec_zone_unactivable: RSet<UItemId>,
    pub(in crate::svc::vast) sec_zone_effect: RMapRMap<UItemId, REffectId, EffectSecZoneInfo>,
    pub(in crate::svc::vast) mods_active: RSet<UItemId>,
    pub(in crate::svc::vast) mods_rigs_svcs_vs_ship_kind: RMap<UItemId, ValShipKind>,
    pub(in crate::svc::vast) stopped_effects: RMapRSet<EffectSpec, EffectSpec>,
    pub(in crate::svc::vast) blockable_assistance: RMapRSet<UItemId, EffectSpec>,
    pub(in crate::svc::vast) blockable_offense: RMapRSet<UItemId, EffectSpec>,
    pub(in crate::svc::vast) resist_immunity: RMapRSet<AttrSpec, EffectSpec>,
    pub(in crate::svc::vast) projectee_filter: RMapRMap<EffectSpec, UItemId, RItemListId>,
    pub(in crate::svc::vast) cap_consumers_all: RMap<UItemId, Vec<RAttrId>>,
    // Stats-related - damage output
    pub(in crate::svc::vast) dmg_normal: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<DmgKinds<PValue>>>,
    pub(in crate::svc::vast) dmg_breacher: RMapRMap<UItemId, REffectId, NBreacherDmgGetter>,
    // Stats-related - mining output
    pub(in crate::svc::vast) mining_ore: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<MiningAmount>>,
    pub(in crate::svc::vast) mining_ice: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<MiningAmount>>,
    pub(in crate::svc::vast) mining_gas: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<MiningAmount>>,
    // Stats-related - RR output
    pub(in crate::svc::vast) orr_shield: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    pub(in crate::svc::vast) orr_armor: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    pub(in crate::svc::vast) orr_hull: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    // Stats-related - misc output
    pub(in crate::svc::vast) out_neuts: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    pub(in crate::svc::vast) out_cap: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<PValue>>,
    // Stats-related - local active tank
    pub(in crate::svc::vast) lr_shield: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<PValue>>,
    pub(in crate::svc::vast) lr_shield_limitable: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<PValue>>,
    pub(in crate::svc::vast) lr_armor: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<PValue>>,
    pub(in crate::svc::vast) lr_armor_limitable: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<PValue>>,
    pub(in crate::svc::vast) lr_hull: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<PValue>>,
    // Stats-related - cap
    pub(in crate::svc::vast) cap_consumers_active: RMapRMap<UItemId, REffectId, RAttrId>,
    pub(in crate::svc::vast) cap_injects: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<PValue>>,
    // Stats-related - misc
    pub(in crate::svc::vast) aggro_effects: RSet<EffectSpec>,
}
impl VastFitData {
    pub(in crate::svc) fn new() -> Self {
        Self {
            // Validation-related
            mods_svcs_online: RSet::new(),
            rigs_offline_calibration: RMap::new(),
            drones_volume: RMap::new(),
            drones_bandwidth: RMap::new(),
            drones_online_bandwidth: RMap::new(),
            fighters_volume: RMap::new(),
            fighters_online: RSet::new(),
            light_fighters: RSet::new(),
            light_fighters_online: RSet::new(),
            heavy_fighters: RSet::new(),
            heavy_fighters_online: RSet::new(),
            support_fighters: RSet::new(),
            support_fighters_online: RSet::new(),
            st_light_fighters: RSet::new(),
            st_light_fighters_online: RSet::new(),
            st_heavy_fighters: RSet::new(),
            st_heavy_fighters_online: RSet::new(),
            st_support_fighters: RSet::new(),
            st_support_fighters_online: RSet::new(),
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
            charge_group: RMap::new(),
            charge_cont_group: RMap::new(),
            charge_size: RMap::new(),
            charge_volume: RMap::new(),
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
            sec_zone_effect: RMapRMap::new(),
            mods_active: RSet::new(),
            mods_rigs_svcs_vs_ship_kind: RMap::new(),
            stopped_effects: RMapRSet::new(),
            projectee_filter: RMapRMap::new(),
            blockable_assistance: RMapRSet::new(),
            blockable_offense: RMapRSet::new(),
            resist_immunity: RMapRSet::new(),
            cap_consumers_all: RMap::new(),
            // Stats-related - damage output
            dmg_normal: RMapRMap::new(),
            dmg_breacher: RMapRMap::new(),
            // Stats-related - mining output
            mining_ore: RMapRMap::new(),
            mining_ice: RMapRMap::new(),
            mining_gas: RMapRMap::new(),
            // Stats-related - RR output
            orr_shield: RMapRMap::new(),
            orr_armor: RMapRMap::new(),
            orr_hull: RMapRMap::new(),
            // Stats-related - misc output
            out_neuts: RMapRMap::new(),
            out_cap: RMapRMap::new(),
            // Stats-related - local active tank
            lr_shield: RMapRMap::new(),
            lr_shield_limitable: RMapRMap::new(),
            lr_armor: RMapRMap::new(),
            lr_armor_limitable: RMapRMap::new(),
            lr_hull: RMapRMap::new(),
            // Stats-related - cap
            cap_consumers_active: RMapRMap::new(),
            cap_injects: RMapRMap::new(),
            // Stats-related - misc
            aggro_effects: RSet::new(),
        }
    }
}
