//! Adapted data generator
use crate::{
    ad,
    defs::{EAttrId, EAttrUnitId, EBuffId, EItemCatId, EItemGrpId, Rational},
    ec, ed,
    src::SrcInitError,
    util::StMap,
};

mod clean;
mod conv;
mod custom;
mod fetch;
mod norm;
mod pk;
mod rels;
mod valid;

/// Fetch EVE data and generate adapted data out of it
#[tracing::instrument(name = "adg", level = "trace", skip_all)]
pub(crate) fn generate_adapted_data(e_handler: &dyn ed::EveDataHandler) -> Result<ad::AData, SrcInitError> {
    let mut g_data = GData::new();
    let mut g_supp = GSupport::new();
    let mut a_data = ad::AData::new();
    fetch::fetch_data(e_handler, &mut g_data).map_err(|e| SrcInitError::EveDataFetchFailed(e.to_string()))?;
    pk::dedup_pks(&mut g_data);
    norm::normalize(&mut g_data);
    g_supp.fill(&g_data);
    clean::clean_unused(&mut g_data, &g_supp).map_err(|e| SrcInitError::EveDataCleanupFailed(e.to_string()))?;
    valid::validate(&mut g_data, &g_supp);
    conv::convert(&g_data, &g_supp, &mut a_data);
    custom::customize(&mut a_data);
    Ok(a_data)
}

/// Container for primary data, used internally by the generator.
pub(in crate::adg) struct GData {
    pub(in crate::adg) items: Vec<ed::EItem>,
    pub(in crate::adg) groups: Vec<ed::EItemGroup>,
    pub(in crate::adg) attrs: Vec<ed::EAttr>,
    pub(in crate::adg) item_attrs: Vec<ed::EItemAttr>,
    pub(in crate::adg) effects: Vec<ed::EEffect>,
    pub(in crate::adg) item_effects: Vec<ed::EItemEffect>,
    pub(in crate::adg) abils: Vec<ed::EFighterAbil>,
    pub(in crate::adg) item_abils: Vec<ed::EItemFighterAbil>,
    pub(in crate::adg) buffs: Vec<ed::EBuff>,
    pub(in crate::adg) item_srqs: Vec<ed::EItemSkillReq>,
    pub(in crate::adg) muta_items: Vec<ed::EMutaItemConv>,
    pub(in crate::adg) muta_attrs: Vec<ed::EMutaAttrMod>,
}
impl GData {
    pub(in crate::adg) fn new() -> Self {
        Self {
            items: Vec::new(),
            groups: Vec::new(),
            attrs: Vec::new(),
            item_attrs: Vec::new(),
            effects: Vec::new(),
            item_effects: Vec::new(),
            abils: Vec::new(),
            item_abils: Vec::new(),
            buffs: Vec::new(),
            item_srqs: Vec::new(),
            muta_items: Vec::new(),
            muta_attrs: Vec::new(),
        }
    }
}

/// Container for auxiliary data.
pub(in crate::adg) struct GSupport {
    pub(in crate::adg) attr_unit_map: StMap<EAttrId, EAttrUnitId>,
    pub(in crate::adg) grp_cat_map: StMap<EItemGrpId, EItemCatId>,
    pub(in crate::adg) eff_buff_map: StMap<EBuffId, ad::AEffectBuffInfo>,
    pub(in crate::adg) eff_charge_map: StMap<EBuffId, ad::AEffectChargeInfo>,
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
        self.fill_attr_unit_map(&g_data);
        self.fill_grp_cat_map(&g_data);
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
                ad::AEffectBuffInfo::new(ad::AEffectBuffSrc::DefaultAttrs, ad::AEffectBuffScope::FleetShips),
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
                ad::AEffectBuffInfo::new(ad::AEffectBuffSrc::DefaultAttrs, ad::AEffectBuffScope::Everything),
            );
        }
        // Buffs which affect only ships, and which rely on standard on-item attributes
        self.eff_buff_map.insert(
            ec::effects::MOD_TITAN_EFFECT_GENERATOR,
            ad::AEffectBuffInfo::new(ad::AEffectBuffSrc::DefaultAttrs, ad::AEffectBuffScope::Ships),
        );
        // Bursts with hardcoded IDs
        self.eff_buff_map.insert(
            ec::effects::DOOMSDAY_AOE_WEB,
            ad::AEffectBuffInfo::new(
                ad::AEffectBuffSrc::Customized(vec![ad::AEffectBuffSrcCustom::AffectorVal(
                    ec::buffs::STASIS_WEBIFICATION_BURST,
                    ec::attrs::SPEED_FACTOR,
                )]),
                ad::AEffectBuffScope::Everything,
            ),
        );
        // Full hardcode
        self.eff_buff_map.insert(
            ec::effects::DEBUFF_LANCE,
            ad::AEffectBuffInfo::new(
                ad::AEffectBuffSrc::Customized(vec![
                    ad::AEffectBuffSrcCustom::HardcodedVal(
                        ec::buffs::REMOTE_REPAIR_IMPEDANCE,
                        Rational::from_integer(-50),
                    ),
                    ad::AEffectBuffSrcCustom::HardcodedVal(ec::buffs::WARP_PENALTY, Rational::from_integer(100)),
                    ad::AEffectBuffSrcCustom::HardcodedVal(ec::buffs::DISALLOW_DOCK_JUMP, Rational::from_integer(1)),
                    ad::AEffectBuffSrcCustom::HardcodedVal(ec::buffs::DISALLOW_TETHER, Rational::from_integer(1)),
                ]),
                ad::AEffectBuffScope::Everything,
            ),
        );
    }
    fn fill_eff_charge_map(&mut self) {
        // Attempt to run effects on default launcher effect just for stasis webification probes
        self.eff_charge_map
            .insert(ec::effects::USE_MISSILES, ad::AEffectChargeInfo::Loaded);
        // LR fighter bombs
        self.eff_charge_map.insert(
            ec::effects::FTR_ABIL_BOMB,
            ad::AEffectChargeInfo::Attr(ec::attrs::FTR_ABIL_BOMB_TYPE),
        );
    }
}
