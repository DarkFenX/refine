use crate::{
    ad::{AItemGrpId, AItemId},
    misc::{AttrSpec, EffectSpec},
    nd::{
        NEffectBreacherOutputGetter, NEffectDmgOutputGetter, NEffectEcmOutputGetter, NEffectGeneralOutputGetter,
        NEffectMiningOutputGetter,
    },
    num::{Count, PValue, SkillLevel, SlotIndex, Value},
    rd::{REffectId, REffectLocalOpcSpec, REffectProjOpcSpec, RItemListId, RItemShipLimit},
    svc::vast::{
        ValFighterSquadSizeFighterInfo, ValItemKindItemInfo, ValModuleStateModuleInfo, ValShipKind, ValSrqSkillInfo,
        validators::EffectSecZoneInfo,
    },
    ud::{UFitId, UItemId},
    util::{ArenaSecUnchecked, RMap, RMapRMap, RMapRMapRMap, RMapRSet, RSet},
};

// Vast stands for VAlidation and STats.
#[derive(Clone)]
pub(in crate::svc) struct Vast {
    pub(super) fit_datas: ArenaSecUnchecked<UFitId, VastFitData>,
    pub(in crate::svc::vast) not_loaded: RSet<UItemId>,
    // Stats-related - incoming remote reps
    pub(in crate::svc::vast) irr_shield:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) irr_shield_limitable:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) irr_armor:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) irr_armor_limitable:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) irr_hull:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    // Stats-related - cap
    pub(in crate::svc::vast) in_cap:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) in_neuts:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    // Stats-related - ewar
    pub(in crate::svc::vast) in_ecm:
        RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectEcmOutputGetter>>,
}
impl Vast {
    pub(in crate::svc) fn new() -> Self {
        Self {
            fit_datas: ArenaSecUnchecked::new(),
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
    pub(in crate::svc) fn get_fit_data(&self, fit_uid: UFitId) -> &VastFitData {
        self.fit_datas.get(fit_uid)
    }
    pub(in crate::svc::vast) fn get_fit_data_mut(&mut self, fit_uid: UFitId) -> &mut VastFitData {
        self.fit_datas.get_mut(fit_uid)
    }
}

#[derive(Clone)]
pub(in crate::svc) struct VastFitData {
    // Validation-related - resources
    pub(in crate::svc::vast) mods_svcs_online: RSet<UItemId>,
    pub(in crate::svc::vast) rigs_offline_calibration: RMap<UItemId, Value>,
    pub(in crate::svc::vast) drones_volume: RMap<UItemId, PValue>,
    pub(in crate::svc::vast) drones_bandwidth: RMap<UItemId, Value>,
    pub(in crate::svc::vast) drones_online_bandwidth: RMap<UItemId, Value>,
    pub(in crate::svc::vast) fighters_volume: RMap<UItemId, PValue>,
    // Validation-related - slots
    pub(in crate::svc::vast) mods_turret: RSet<UItemId>,
    pub(in crate::svc::vast) mods_launcher: RSet<UItemId>,
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
    // Validation-related - slot index
    pub(in crate::svc::vast) slotted_implants: RMapRSet<SlotIndex, UItemId>,
    pub(in crate::svc::vast) slotted_boosters: RMapRSet<SlotIndex, UItemId>,
    pub(in crate::svc::vast) slotted_subsystems: RMapRSet<SlotIndex, UItemId>,
    // Validation-related - restrictions between ship type/attribute and its items
    pub(in crate::svc::vast) ship_limited_items: RMap<UItemId, RItemShipLimit>,
    pub(in crate::svc::vast) rigs_rig_size: RMap<UItemId, Option<Value>>,
    pub(in crate::svc::vast) mods_rigs_svcs_vs_ship_kind: RMap<UItemId, ValShipKind>,
    pub(in crate::svc::vast) mods_capital: RMap<UItemId, PValue>,
    pub(in crate::svc::vast) drone_group_limit: Vec<AItemGrpId>,
    pub(in crate::svc::vast) drone_groups: RMap<UItemId, AItemGrpId>,
    // Validation-related - max type/group
    pub(in crate::svc::vast) mods_svcs_max_type_fitted: RMapRMap<AItemId, UItemId, Count>,
    pub(in crate::svc::vast) mods_svcs_rigs_max_group_fitted_all: RMapRSet<AItemGrpId, UItemId>,
    pub(in crate::svc::vast) mods_svcs_rigs_max_group_fitted_limited: RMap<UItemId, AItemGrpId>,
    pub(in crate::svc::vast) mods_svcs_max_group_online_all: RMapRSet<AItemGrpId, UItemId>,
    pub(in crate::svc::vast) mods_svcs_max_group_online_limited: RMap<UItemId, AItemGrpId>,
    pub(in crate::svc::vast) mods_max_group_active_all: RMapRSet<AItemGrpId, UItemId>,
    pub(in crate::svc::vast) mods_max_group_active_limited: RMap<UItemId, AItemGrpId>,
    // Validation-related - module-charge restrictions
    pub(in crate::svc::vast) charge_group: RMap<UItemId, UItemId>,
    pub(in crate::svc::vast) charge_cont_group: RMap<UItemId, UItemId>,
    pub(in crate::svc::vast) charge_size: RMap<UItemId, UItemId>,
    pub(in crate::svc::vast) charge_volume: RMap<UItemId, UItemId>,
    // Validation-related - projection
    pub(in crate::svc::vast) projectee_filter: RMapRMap<EffectSpec, UItemId, RItemListId>,
    pub(in crate::svc::vast) blockable_assistance: RMapRSet<UItemId, EffectSpec>,
    pub(in crate::svc::vast) blockable_offense: RMapRSet<UItemId, EffectSpec>,
    pub(in crate::svc::vast) resist_immunity: RMapRSet<AttrSpec, EffectSpec>,
    pub(in crate::svc::vast) stopped_effects: RMapRSet<EffectSpec, EffectSpec>,
    // Validation-related - skills
    pub(in crate::svc::vast) srqs_skill_item_map: RMapRSet<AItemId, UItemId>,
    pub(in crate::svc::vast) srqs_missing: RMap<UItemId, RMap<AItemId, ValSrqSkillInfo>>,
    pub(in crate::svc::vast) overload_td_lvl: RMap<UItemId, SkillLevel>,
    // Validation-related - security zone
    pub(in crate::svc::vast) sec_zone_fitted: RSet<UItemId>,
    pub(in crate::svc::vast) sec_zone_fitted_wspace_banned: RSet<UItemId>,
    pub(in crate::svc::vast) sec_zone_online_class: RMap<UItemId, Value>,
    pub(in crate::svc::vast) sec_zone_active: RSet<UItemId>,
    pub(in crate::svc::vast) sec_zone_unonlineable_class: RMap<UItemId, Value>,
    pub(in crate::svc::vast) sec_zone_unactivable: RSet<UItemId>,
    pub(in crate::svc::vast) sec_zone_effect: RMapRMap<UItemId, REffectId, EffectSecZoneInfo>,
    // Validation-related - misc
    pub(in crate::svc::vast) not_loaded: RSet<UItemId>,
    pub(in crate::svc::vast) item_kind: RMap<UItemId, ValItemKindItemInfo>,
    pub(in crate::svc::vast) mods_state: RMap<UItemId, ValModuleStateModuleInfo>,
    pub(in crate::svc::vast) mods_active: RSet<UItemId>,
    pub(in crate::svc::vast) mods_active_cloaks: RSet<EffectSpec>,
    pub(in crate::svc::vast) mods_cap_consumers: RSet<UItemId>,
    pub(in crate::svc::vast) fighter_squad_size: RMap<UItemId, ValFighterSquadSizeFighterInfo>,
    pub(in crate::svc::vast) mods_fitted_cloaks: Count,
    // Stats-related - damage output
    pub(in crate::svc::vast) dmg_normal: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectDmgOutputGetter>>,
    pub(in crate::svc::vast) dmg_breacher:
        RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectBreacherOutputGetter>>,
    // Stats-related - mining output
    pub(in crate::svc::vast) mining_ore: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectMiningOutputGetter>>,
    pub(in crate::svc::vast) mining_ice: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectMiningOutputGetter>>,
    pub(in crate::svc::vast) mining_gas: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectMiningOutputGetter>>,
    // Stats-related - RR output
    pub(in crate::svc::vast) orr_shield: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) orr_armor: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) orr_hull: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    // Stats-related - misc output
    pub(in crate::svc::vast) out_neuts: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) out_cap: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    // Stats-related - local active tank
    pub(in crate::svc::vast) lr_shield: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) lr_shield_limitable:
        RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) lr_armor: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) lr_armor_limitable:
        RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) lr_hull: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    // Stats-related - cap
    pub(in crate::svc::vast) cap_consumers:
        RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) cap_nosfs: RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(in crate::svc::vast) cap_injects: RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    // Stats-related - effect-based restrictions
    pub(in crate::svc::vast) effects_aggro: RSet<EffectSpec>,
    pub(in crate::svc::vast) mod_effects_disallow_cloak: RSet<EffectSpec>,
    pub(in crate::svc::vast) mod_effects_disallow_warp: RSet<EffectSpec>,
    pub(in crate::svc::vast) mod_effects_disallow_jump_gate: RSet<EffectSpec>,
    pub(in crate::svc::vast) mod_effects_disallow_jump_wh: RSet<EffectSpec>,
    pub(in crate::svc::vast) mod_effects_disallow_jump_drive: RSet<EffectSpec>,
    pub(in crate::svc::vast) mod_effects_disallow_dock: RSet<EffectSpec>,
}
impl VastFitData {
    pub(in crate::svc) fn new() -> Self {
        Self {
            // Validation-related - resources
            mods_svcs_online: RSet::new(),
            rigs_offline_calibration: RMap::new(),
            drones_volume: RMap::new(),
            drones_bandwidth: RMap::new(),
            drones_online_bandwidth: RMap::new(),
            fighters_volume: RMap::new(),
            // Validation-related - slots
            mods_turret: RSet::new(),
            mods_launcher: RSet::new(),
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
            // Validation-related - slot index
            slotted_implants: RMapRSet::new(),
            slotted_boosters: RMapRSet::new(),
            slotted_subsystems: RMapRSet::new(),
            // Validation-related - restrictions between ship type/attribute and its items
            ship_limited_items: RMap::new(),
            rigs_rig_size: RMap::new(),
            mods_rigs_svcs_vs_ship_kind: RMap::new(),
            mods_capital: RMap::new(),
            drone_group_limit: Vec::new(),
            drone_groups: RMap::new(),
            // Validation-related - max type/group
            mods_svcs_max_type_fitted: RMapRMap::new(),
            mods_svcs_rigs_max_group_fitted_all: RMapRSet::new(),
            mods_svcs_rigs_max_group_fitted_limited: RMap::new(),
            mods_svcs_max_group_online_all: RMapRSet::new(),
            mods_svcs_max_group_online_limited: RMap::new(),
            mods_max_group_active_all: RMapRSet::new(),
            mods_max_group_active_limited: RMap::new(),
            // Validation-related - module-charge restrictions
            charge_group: RMap::new(),
            charge_cont_group: RMap::new(),
            charge_size: RMap::new(),
            charge_volume: RMap::new(),
            // Validation-related - projection
            projectee_filter: RMapRMap::new(),
            blockable_assistance: RMapRSet::new(),
            blockable_offense: RMapRSet::new(),
            resist_immunity: RMapRSet::new(),
            stopped_effects: RMapRSet::new(),
            // Validation-related - skills
            srqs_skill_item_map: RMapRSet::new(),
            srqs_missing: RMap::new(),
            overload_td_lvl: RMap::new(),
            // Validation-related - security zone
            sec_zone_fitted: RSet::new(),
            sec_zone_fitted_wspace_banned: RSet::new(),
            sec_zone_online_class: RMap::new(),
            sec_zone_active: RSet::new(),
            sec_zone_unonlineable_class: RMap::new(),
            sec_zone_unactivable: RSet::new(),
            sec_zone_effect: RMapRMap::new(),
            // Validation-related - misc
            not_loaded: RSet::new(),
            item_kind: RMap::new(),
            mods_state: RMap::new(),
            mods_active: RSet::new(),
            mods_active_cloaks: RSet::new(),
            mods_cap_consumers: RSet::new(),
            fighter_squad_size: RMap::new(),
            mods_fitted_cloaks: Count::ZERO,
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
            cap_nosfs: RMapRMap::new(),
            out_cap: RMapRMap::new(),
            // Stats-related - local active tank
            lr_shield: RMapRMap::new(),
            lr_shield_limitable: RMapRMap::new(),
            lr_armor: RMapRMap::new(),
            lr_armor_limitable: RMapRMap::new(),
            lr_hull: RMapRMap::new(),
            // Stats-related - cap
            cap_consumers: RMapRMap::new(),
            cap_injects: RMapRMap::new(),
            // Stats-related - effect-based restrictions
            effects_aggro: RSet::new(),
            mod_effects_disallow_cloak: RSet::new(),
            mod_effects_disallow_warp: RSet::new(),
            mod_effects_disallow_jump_gate: RSet::new(),
            mod_effects_disallow_jump_wh: RSet::new(),
            mod_effects_disallow_jump_drive: RSet::new(),
            mod_effects_disallow_dock: RSet::new(),
        }
    }
}
