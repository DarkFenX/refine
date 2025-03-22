use ordered_float::OrderedFloat as OF;

use crate::{ac, ad, adg::data::GData, ec, ed, util::StMap};

/// Container for auxiliary data.
pub(in crate::adg) struct GSupport {
    pub(in crate::adg) attr_unit_map: StMap<ed::EAttrId, ed::EAttrUnitId>,
    pub(in crate::adg) grp_cat_map: StMap<ed::EItemGrpId, ed::EItemCatId>,
    pub(in crate::adg) eff_buff_map: StMap<ed::EEffectId, ad::AEffectBuffInfo>,
    pub(in crate::adg) eff_charge_map: StMap<ed::EEffectId, ad::AEffectChargeInfo>,
}
impl GSupport {
    pub(in crate::adg) fn new() -> Self {
        Self {
            attr_unit_map: StMap::new(),
            grp_cat_map: StMap::new(),
            eff_buff_map: StMap::new(),
            eff_charge_map: StMap::new(),
        }
    }
    pub(in crate::adg) fn fill(&mut self, g_data: &GData) {
        self.fill_attr_unit_map(g_data);
        self.fill_grp_cat_map(g_data);
        self.fill_eff_buff_map();
        self.fill_eff_charge_map();
    }
    fn fill_attr_unit_map(&mut self, g_data: &GData) {
        for attr in g_data.attrs.iter() {
            if let Some(unit) = attr.unit_id {
                self.attr_unit_map.insert(attr.id, unit);
            }
        }
    }
    fn fill_grp_cat_map(&mut self, g_data: &GData) {
        for grp in g_data.groups.iter() {
            self.grp_cat_map.insert(grp.id, grp.category_id);
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
    fn fill_eff_charge_map(&mut self) {
        // Attempt to run effects on default launcher effect just for stasis webification probes
        self.eff_charge_map
            .insert(ec::effects::USE_MISSILES, ad::AEffectChargeInfo::Loaded);
        // LR fighter bombs
        self.eff_charge_map.insert(
            ec::effects::FTR_ABIL_BOMB,
            ad::AEffectChargeInfo::Attr(ac::attrs::FTR_ABIL_BOMB_TYPE),
        );
    }
}
