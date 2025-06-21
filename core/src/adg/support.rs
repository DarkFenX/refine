use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad, ec, ed,
    util::{RMap, RMapRSet, RSet},
};

/// Container for auxiliary data.
pub(in crate::adg) struct GSupport {
    pub(in crate::adg) grp_cat_map: RMap<ed::EItemGrpId, ed::EItemCatId>,
    pub(in crate::adg) rendered_type_lists: RMap<ed::EItemListId, RSet<ad::AItemId>>,
    pub(in crate::adg) attr_unit_map: RMap<ed::EAttrId, ed::EAttrUnitId>,
    pub(in crate::adg) eff_buff_map: RMap<ed::EEffectId, ad::AEffectBuffInfo>,
    pub(in crate::adg) eff_charge_map: RMap<ed::EEffectId, ad::AEffectChargeInfo>,
}
impl GSupport {
    pub(in crate::adg) fn new() -> Self {
        Self {
            grp_cat_map: RMap::new(),
            rendered_type_lists: RMap::new(),
            attr_unit_map: RMap::new(),
            eff_buff_map: RMap::new(),
            eff_charge_map: RMap::new(),
        }
    }
    pub(in crate::adg) fn fill(&mut self, e_data: &ed::EData) {
        self.fill_grp_cat_map(e_data);
        self.fill_rendered_type_lists(e_data);
        self.fill_attr_unit_map(e_data);
        self.fill_eff_buff_map();
        self.fill_eff_charge_map();
    }
    fn fill_grp_cat_map(&mut self, e_data: &ed::EData) {
        for grp in e_data.groups.data.iter() {
            self.grp_cat_map.insert(grp.id, grp.category_id);
        }
    }
    fn fill_rendered_type_lists(&mut self, e_data: &ed::EData) {
        let mut types_by_grp = RMapRSet::new();
        for item in e_data.items.data.iter() {
            types_by_grp.add_entry(item.group_id, item.id);
        }
        let mut types_by_cat = RMapRSet::new();
        for group in e_data.groups.data.iter() {
            types_by_cat.extend_entries(group.category_id, types_by_grp.get(&group.id).copied());
        }
        for item_list in &e_data.item_lists.data {
            let mut includes = RSet::new();
            includes.extend(item_list.included_item_ids.iter().copied());
            for included_grp_id in item_list.included_grp_ids.iter() {
                includes.extend(types_by_grp.get(included_grp_id).copied());
            }
            for included_cat_id in item_list.included_cat_ids.iter() {
                includes.extend(types_by_cat.get(included_cat_id).copied());
            }
            let mut excludes = RSet::new();
            excludes.extend(item_list.excluded_item_ids.iter().copied());
            for excluded_grp_id in item_list.excluded_grp_ids.iter() {
                excludes.extend(types_by_grp.get(excluded_grp_id).copied());
            }
            for excluded_cat_id in item_list.included_cat_ids.iter() {
                excludes.extend(types_by_cat.get(excluded_cat_id).copied());
            }
            self.rendered_type_lists
                .insert(item_list.id, includes.difference(&excludes).copied().collect());
        }
    }
    fn fill_eff_buff_map(&mut self) {
        // Fleet buffs which rely on standard on-item attributes to define buffs
        for effect_id in [
            ec::effects::MOD_BONUS_WARFARE_LINK_ARMOR,
            ec::effects::MOD_BONUS_WARFARE_LINK_INFO,
            ec::effects::MOD_BONUS_WARFARE_LINK_MINING,
            ec::effects::MOD_BONUS_WARFARE_LINK_SHIELD,
            ec::effects::MOD_BONUS_WARFARE_LINK_SKIRMISH,
        ] {
            self.eff_buff_map.insert(
                effect_id,
                ad::AEffectBuffInfo {
                    source: ad::AEffectBuffSrc::DefaultAttrs,
                    scope: ad::AEffectBuffScope::FleetShips,
                },
            );
        }
        // Buffs which affect everything, and which rely on standard on-item attributes
        for effect_id in [
            ec::effects::WEATHER_ELECTRIC_STORM,
            ec::effects::WEATHER_INFERNAL,
            ec::effects::WEATHER_CAUSTIC_TOXIN,
            ec::effects::WEATHER_XENON_GAS,
            ec::effects::WEATHER_DARKNESS,
            ec::effects::AOE_BEACON_BIOLUMINESCENCE_CLOUD,
            ec::effects::AOE_BEACON_CAUSTIC_CLOUD,
            ec::effects::AOE_BEACON_FILAMENT_CLOUD,
            ec::effects::AOE_BEACON_PULSE_01, // Tracking towers in the abyss
        ] {
            self.eff_buff_map.insert(
                effect_id,
                ad::AEffectBuffInfo {
                    source: ad::AEffectBuffSrc::DefaultAttrs,
                    scope: ad::AEffectBuffScope::Everything,
                },
            );
        }
        // Buffs which affect only ships, and which rely on standard on-item attributes
        self.eff_buff_map.insert(
            ec::effects::MOD_TITAN_EFFECT_GENERATOR,
            ad::AEffectBuffInfo {
                source: ad::AEffectBuffSrc::DefaultAttrs,
                scope: ad::AEffectBuffScope::Ships,
            },
        );
        // Bursts with hardcoded IDs
        self.eff_buff_map.insert(
            ec::effects::DOOMSDAY_AOE_WEB,
            ad::AEffectBuffInfo {
                source: ad::AEffectBuffSrc::Customized(vec![ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::STASIS_WEBIFICATION_BURST,
                    ac::attrs::SPEED_FACTOR,
                )]),
                scope: ad::AEffectBuffScope::Everything,
            },
        );
        // Full hardcode
        self.eff_buff_map.insert(
            ec::effects::DEBUFF_LANCE,
            ad::AEffectBuffInfo {
                source: ad::AEffectBuffSrc::Customized(vec![
                    ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::REMOTE_REPAIR_IMPEDANCE, OF(-50.0)),
                    ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::WARP_PENALTY, OF(100.0)),
                    ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_DOCK_JUMP, OF(1.0)),
                    ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_TETHER, OF(1.0)),
                ]),
                scope: ad::AEffectBuffScope::Everything,
            },
        );
    }
    fn fill_attr_unit_map(&mut self, e_data: &ed::EData) {
        for attr in e_data.attrs.data.iter() {
            if let Some(unit) = attr.unit_id {
                self.attr_unit_map.insert(attr.id, unit);
            }
        }
    }
    fn fill_eff_charge_map(&mut self) {
        for effect_id in [
            ec::effects::USE_MISSILES,
            // Ancillary modules
            ec::effects::FUELED_SHIELD_BOOSTING,
            ec::effects::FUELED_ARMOR_REPAIR,
            ec::effects::SHIP_MODULE_ARSR,
            ec::effects::SHIP_MODULE_ARAR,
        ] {
            self.eff_charge_map.insert(effect_id, ad::AEffectChargeInfo::Loaded);
        }
        // LR fighter bombs
        self.eff_charge_map.insert(
            ec::effects::FTR_ABIL_BOMB,
            ad::AEffectChargeInfo::Attr(ac::attrs::FTR_ABIL_BOMB_TYPE),
        );
    }
}
