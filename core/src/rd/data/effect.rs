use crate::{
    ad::{AAttrId, AEffect, AEffectBuffInfo, AEffectCatId, AEffectId, AEffectModifier, AState},
    nd::{
        N_EFFECT_MAP, NBreacherDmgGetter, NCalcCustomizer, NCapBoostGetter, NDmgKindGetter, NEcmGetter, NEffectCharge,
        NEffectChargeLoc, NEffectHc, NLocalRepGetter, NNeutGetter, NNormalDmgGetter, NProjMultGetter, NRemoteRepGetter,
        NSpoolResolver,
    },
    rd::{REffectKey, RItem},
    util::{GetId, Named, RMap},
};

// Represents an effect.
//
// Effects are higher-level modification descriptors, as opposed to modifiers, which are
// lower-level. An effect can contain any number of modifiers under a single roof, accompanied by
// extra effect-wide properties.
pub(crate) struct REffect {
    effect_key: REffectKey,
    a_effect: AEffect,
    n_effect_hc: NEffectHc,
    // Extra data extracted from adapted effect and hardcoded data
    is_active_with_duration: bool,
    modifier_proj_attr_ids: [Option<AAttrId>; 2],
    // Fields which need slab keys to be filled
    stopped_effect_keys: Vec<REffectKey>,
}
impl REffect {
    pub(in crate::rd) fn new(effect_key: REffectKey, a_effect: AEffect) -> Self {
        let n_effect = N_EFFECT_MAP.get(&a_effect.id);
        let is_active_with_duration = a_effect.state == AState::Active && a_effect.duration_attr_id.is_some();
        let modifier_proj_attr_ids = n_effect
            .and_then(|v| v.modifier_proj_attrs_getter)
            .map(|get_proj_attrs| get_proj_attrs(&a_effect))
            .unwrap_or_default();
        Self {
            effect_key,
            a_effect,
            n_effect_hc: n_effect.map(|n_effect| n_effect.hc).unwrap_or_default(),
            is_active_with_duration,
            modifier_proj_attr_ids,
            stopped_effect_keys: Vec::new(),
        }
    }
    pub(in crate::rd) fn fill_key_dependents(&mut self, effect_id_key_map: &RMap<AEffectId, REffectKey>) {
        self.stopped_effect_keys.extend(
            self.a_effect
                .stoped_effect_ids
                .iter()
                .filter_map(|v| effect_id_key_map.get(v)),
        );
    }
    // Methods which expose adapted effect info
    pub(crate) fn get_category(&self) -> AEffectCatId {
        self.a_effect.category
    }
    pub(crate) fn get_state(&self) -> AState {
        self.a_effect.state
    }
    pub(crate) fn is_assist(&self) -> bool {
        self.a_effect.is_assist
    }
    pub(crate) fn is_offense(&self) -> bool {
        self.a_effect.is_offense
    }
    pub(crate) fn is_usable_in_hisec(&self) -> Option<bool> {
        self.a_effect.hisec
    }
    pub(crate) fn is_usable_in_lowsec(&self) -> Option<bool> {
        self.a_effect.lowsec
    }
    pub(crate) fn get_discharge_attr_id(&self) -> Option<AAttrId> {
        self.a_effect.discharge_attr_id
    }
    pub(crate) fn get_duration_attr_id(&self) -> Option<AAttrId> {
        self.a_effect.duration_attr_id
    }
    pub(crate) fn get_range_attr_id(&self) -> Option<AAttrId> {
        self.a_effect.range_attr_id
    }
    pub(crate) fn get_falloff_attr_id(&self) -> Option<AAttrId> {
        self.a_effect.falloff_attr_id
    }
    pub(crate) fn get_track_attr_id(&self) -> Option<AAttrId> {
        self.a_effect.track_attr_id
    }
    pub(crate) fn get_chance_attr_id(&self) -> Option<AAttrId> {
        self.a_effect.chance_attr_id
    }
    pub(crate) fn get_resist_attr_id(&self) -> Option<AAttrId> {
        self.a_effect.resist_attr_id
    }
    pub(crate) fn get_mods(&self) -> &Vec<AEffectModifier> {
        &self.a_effect.mods
    }
    // Methods which expose hardcoded effect info
    pub(crate) fn get_charge_info(&self) -> Option<NEffectCharge> {
        self.n_effect_hc.charge
    }
    pub(crate) fn kills_item(&self) -> bool {
        self.n_effect_hc.kills_item
    }
    pub(crate) fn get_calc_customizer(&self) -> Option<NCalcCustomizer> {
        self.n_effect_hc.calc_customizer
    }
    pub(crate) fn get_spool_resolver(&self) -> Option<NSpoolResolver> {
        self.n_effect_hc.spool_resolver
    }
    pub(crate) fn get_modifier_proj_mult_getter(&self) -> Option<NProjMultGetter> {
        self.n_effect_hc.modifier_proj_mult_getter
    }
    pub(crate) fn get_local_shield_rep_opc_getter(&self) -> Option<NLocalRepGetter> {
        self.n_effect_hc.local_shield_rep_opc_getter
    }
    pub(crate) fn get_local_armor_rep_opc_getter(&self) -> Option<NLocalRepGetter> {
        self.n_effect_hc.local_armor_rep_opc_getter
    }
    pub(crate) fn get_local_hull_rep_opc_getter(&self) -> Option<NLocalRepGetter> {
        self.n_effect_hc.local_hull_rep_opc_getter
    }
    pub(crate) fn get_dmg_kind_getter(&self) -> Option<NDmgKindGetter> {
        self.n_effect_hc.dmg_kind_getter
    }
    pub(crate) fn get_normal_dmg_opc_getter(&self) -> Option<NNormalDmgGetter> {
        self.n_effect_hc.normal_dmg_opc_getter
    }
    pub(crate) fn get_breacher_dmg_opc_getter(&self) -> Option<NBreacherDmgGetter> {
        self.n_effect_hc.breacher_dmg_opc_getter
    }
    pub(crate) fn get_remote_shield_rep_opc_getter(&self) -> Option<NRemoteRepGetter> {
        self.n_effect_hc.remote_shield_rep_opc_getter
    }
    pub(crate) fn get_remote_armor_rep_opc_getter(&self) -> Option<NRemoteRepGetter> {
        self.n_effect_hc.remote_armor_rep_opc_getter
    }
    pub(crate) fn get_remote_hull_rep_opc_getter(&self) -> Option<NRemoteRepGetter> {
        self.n_effect_hc.remote_hull_rep_opc_getter
    }
    pub(crate) fn get_remote_cap_rep_opc_getter(&self) -> Option<NRemoteRepGetter> {
        self.n_effect_hc.remote_cap_rep_opc_getter
    }
    pub(crate) fn get_neut_opc_getter(&self) -> Option<NNeutGetter> {
        self.n_effect_hc.neut_opc_getter
    }
    pub(crate) fn get_cap_boost_opc_getter(&self) -> Option<NCapBoostGetter> {
        self.n_effect_hc.cap_boost_opc_getter
    }
    pub(crate) fn get_ecm_opc_getter(&self) -> Option<NEcmGetter> {
        self.n_effect_hc.ecm_opc_getter
    }
    // Methods which expose info generated during runtime
    pub(crate) fn get_key(&self) -> REffectKey {
        self.effect_key
    }
    pub(crate) fn is_active(&self) -> bool {
        self.a_effect.state == AState::Active
    }
    pub(crate) fn is_active_with_duration(&self) -> bool {
        self.is_active_with_duration
    }
    pub(crate) fn get_modifier_proj_attr_ids(&self) -> [Option<AAttrId>; 2] {
        self.modifier_proj_attr_ids
    }
    pub(crate) fn get_stopped_effect_keys(&self) -> &Vec<REffectKey> {
        &self.stopped_effect_keys
    }
    pub(crate) fn get_buff_info(&self) -> Option<&AEffectBuffInfo> {
        self.a_effect.buff_info.as_ref()
    }
    // Misc methods
    pub(crate) fn activates_charge(&self) -> bool {
        let charge_info = match self.n_effect_hc.charge {
            Some(charge_info) => charge_info,
            None => return false,
        };
        if !charge_info.activates_charge {
            return false;
        }
        matches!(charge_info.location, NEffectChargeLoc::Loaded(_))
    }
    pub(crate) fn activates_charge_for_item(&self, item: &RItem) -> bool {
        if !self.activates_charge() {
            return false;
        }
        // Only default effects can activate regular charge
        let defeff_key = match item.get_defeff_key() {
            Some(defeff_key) => defeff_key,
            None => return false,
        };
        defeff_key == self.effect_key
    }
    pub(crate) fn activates_autocharge(&self) -> bool {
        let charge_info = match self.n_effect_hc.charge {
            Some(charge_info) => charge_info,
            None => return false,
        };
        if !charge_info.activates_charge {
            return false;
        }
        matches!(charge_info.location, NEffectChargeLoc::Autocharge(_))
    }
}
impl GetId<AEffectId> for REffect {
    fn get_id(&self) -> AEffectId {
        self.a_effect.id
    }
}
impl Named for REffect {
    fn get_name() -> &'static str {
        "REffect"
    }
}
