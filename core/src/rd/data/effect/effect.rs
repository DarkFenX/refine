use crate::{
    ad::{AAttrId, ABuffId, AEffect, AEffectCatId, AEffectId, AItemListId, AState},
    nd::{
        N_EFFECT_MAP, NBreacherDmgGetter, NCalcCustomizer, NCapInjectGetter, NDmgKindGetter, NEcmGetter,
        NLocalRepGetter, NMiningGetter, NNeutGetter, NNormalDmgGetter, NOutgoingRepGetter, NProjMultGetter,
        NSpoolResolver,
    },
    rd::{
        RAttrKey, RBuffKey, REffectBuff, REffectCharge, REffectChargeLoc, REffectKey, REffectModifier,
        REffectProjecteeFilter, RItem, RItemListKey,
    },
    util::RMap,
};

// Represents an effect.
//
// Effects are higher-level modification descriptors, as opposed to modifiers, which are
// lower-level. An effect can contain any number of modifiers under a single roof, accompanied by
// extra effect-wide properties.
pub(crate) struct REffect {
    pub(crate) key: REffectKey,
    pub(crate) id: AEffectId,
    pub(crate) category: AEffectCatId,
    pub(crate) state: AState,
    pub(crate) is_assist: bool,
    pub(crate) is_offense: bool,
    pub(crate) banned_in_hisec: bool,
    pub(crate) banned_in_lowsec: bool,
    pub(crate) kills_item: bool,
    pub(crate) spool_resolver: Option<NSpoolResolver>,
    pub(crate) calc_customizer: Option<NCalcCustomizer>,
    pub(crate) modifier_proj_mult_getter: Option<NProjMultGetter>,
    // Output getters
    pub(crate) dmg_kind_getter: Option<NDmgKindGetter>,
    pub(crate) normal_dmg_opc_getter: Option<NNormalDmgGetter>,
    pub(crate) breacher_dmg_opc_getter: Option<NBreacherDmgGetter>,
    pub(crate) mining_ore_opc_getter: Option<NMiningGetter>,
    pub(crate) mining_ice_opc_getter: Option<NMiningGetter>,
    pub(crate) mining_gas_opc_getter: Option<NMiningGetter>,
    pub(crate) outgoing_shield_rep_opc_getter: Option<NOutgoingRepGetter>,
    pub(crate) outgoing_armor_rep_opc_getter: Option<NOutgoingRepGetter>,
    pub(crate) outgoing_hull_rep_opc_getter: Option<NOutgoingRepGetter>,
    pub(crate) outgoing_cap_rep_opc_getter: Option<NOutgoingRepGetter>,
    pub(crate) local_shield_rep_opc_getter: Option<NLocalRepGetter>,
    pub(crate) local_armor_rep_opc_getter: Option<NLocalRepGetter>,
    pub(crate) local_hull_rep_opc_getter: Option<NLocalRepGetter>,
    pub(crate) neut_opc_getter: Option<NNeutGetter>,
    pub(crate) cap_inject_getter: Option<NCapInjectGetter>,
    pub(crate) ecm_opc_getter: Option<NEcmGetter>,
    // Fields which depend on slab keys
    pub(crate) modifiers: Vec<REffectModifier>,
    pub(crate) stopped_effect_keys: Vec<REffectKey>,
    pub(crate) buff: Option<REffectBuff>,
    pub(crate) charge: Option<REffectCharge>,
    pub(crate) projectee_filter: Option<REffectProjecteeFilter>,
    pub(crate) modifier_proj_attr_keys: [Option<RAttrKey>; 2],
    pub(crate) discharge_attr_key: Option<RAttrKey>,
    pub(crate) duration_attr_key: Option<RAttrKey>,
    pub(crate) range_attr_key: Option<RAttrKey>,
    pub(crate) falloff_attr_key: Option<RAttrKey>,
    pub(crate) track_attr_key: Option<RAttrKey>,
    pub(crate) chance_attr_key: Option<RAttrKey>,
    pub(crate) resist_attr_key: Option<RAttrKey>,
    pub(crate) is_active_with_duration: bool,
}
impl REffect {
    pub(in crate::rd) fn from_a_effect(effect_key: REffectKey, a_effect: &AEffect) -> Self {
        let n_effect = N_EFFECT_MAP.get(&a_effect.id);
        Self {
            key: effect_key,
            id: a_effect.id,
            category: a_effect.category,
            state: a_effect.state,
            is_assist: a_effect.is_assist,
            is_offense: a_effect.is_offense,
            banned_in_hisec: a_effect.banned_in_hisec,
            banned_in_lowsec: a_effect.banned_in_lowsec,
            kills_item: n_effect.map(|n| n.kills_item).unwrap_or(false),
            spool_resolver: n_effect.and_then(|n| n.spool_resolver),
            calc_customizer: n_effect.and_then(|n| n.calc_customizer),
            modifier_proj_mult_getter: n_effect.and_then(|n| n.modifier_proj_mult_getter),
            // Output getters
            dmg_kind_getter: n_effect.and_then(|n| n.dmg_kind_getter),
            normal_dmg_opc_getter: n_effect.and_then(|n| n.normal_dmg_opc_getter),
            breacher_dmg_opc_getter: n_effect.and_then(|n| n.breacher_dmg_opc_getter),
            mining_ore_opc_getter: n_effect.and_then(|n| n.mining_ore_opc_getter),
            mining_ice_opc_getter: n_effect.and_then(|n| n.mining_ice_opc_getter),
            mining_gas_opc_getter: n_effect.and_then(|n| n.mining_gas_opc_getter),
            outgoing_shield_rep_opc_getter: n_effect.and_then(|n| n.outgoing_shield_rep_opc_getter),
            outgoing_armor_rep_opc_getter: n_effect.and_then(|n| n.outgoing_armor_rep_opc_getter),
            outgoing_hull_rep_opc_getter: n_effect.and_then(|n| n.outgoing_hull_rep_opc_getter),
            outgoing_cap_rep_opc_getter: n_effect.and_then(|n| n.outgoing_cap_rep_opc_getter),
            local_shield_rep_opc_getter: n_effect.and_then(|n| n.local_shield_rep_opc_getter),
            local_armor_rep_opc_getter: n_effect.and_then(|n| n.local_armor_rep_opc_getter),
            local_hull_rep_opc_getter: n_effect.and_then(|n| n.local_hull_rep_opc_getter),
            neut_opc_getter: n_effect.and_then(|n| n.neut_opc_getter),
            cap_inject_getter: n_effect.and_then(|n| n.cap_inject_getter),
            ecm_opc_getter: n_effect.and_then(|n| n.ecm_opc_getter),
            // Fields which depend on slab keys
            modifiers: Default::default(),
            stopped_effect_keys: Default::default(),
            buff: Default::default(),
            charge: Default::default(),
            projectee_filter: Default::default(),
            modifier_proj_attr_keys: Default::default(),
            discharge_attr_key: Default::default(),
            duration_attr_key: Default::default(),
            range_attr_key: Default::default(),
            falloff_attr_key: Default::default(),
            track_attr_key: Default::default(),
            chance_attr_key: Default::default(),
            resist_attr_key: Default::default(),
            is_active_with_duration: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_key_dependents(
        &mut self,
        a_effects: &RMap<AEffectId, AEffect>,
        item_list_id_key_map: &RMap<AItemListId, RItemListKey>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
        effect_id_key_map: &RMap<AEffectId, REffectKey>,
        buff_id_key_map: &RMap<ABuffId, RBuffKey>,
    ) {
        let a_effect = a_effects.get(&self.id).unwrap();
        self.buff = a_effect.buff.as_ref().and_then(|a_effect_buff| {
            REffectBuff::try_from_a_buff(a_effect_buff, item_list_id_key_map, attr_id_key_map, buff_id_key_map)
        });
        self.discharge_attr_key = a_effect
            .discharge_attr_id
            .and_then(|id| attr_id_key_map.get(&id))
            .copied();
        self.duration_attr_key = a_effect
            .duration_attr_id
            .and_then(|id| attr_id_key_map.get(&id))
            .copied();
        self.range_attr_key = a_effect.range_attr_id.and_then(|id| attr_id_key_map.get(&id)).copied();
        self.falloff_attr_key = a_effect
            .falloff_attr_id
            .and_then(|id| attr_id_key_map.get(&id))
            .copied();
        self.falloff_attr_key = a_effect
            .falloff_attr_id
            .and_then(|id| attr_id_key_map.get(&id))
            .copied();
        self.track_attr_key = a_effect.track_attr_id.and_then(|id| attr_id_key_map.get(&id)).copied();
        self.chance_attr_key = a_effect.chance_attr_id.and_then(|id| attr_id_key_map.get(&id)).copied();
        self.resist_attr_key = a_effect.resist_attr_id.and_then(|id| attr_id_key_map.get(&id)).copied();
        self.modifiers.extend(
            a_effect
                .modifiers
                .iter()
                .filter_map(|a_effect_mod| REffectModifier::try_from_a_effect_mod(a_effect_mod, attr_id_key_map)),
        );
        self.stopped_effect_keys.extend(
            a_effect
                .stoped_effect_ids
                .iter()
                .filter_map(|v| effect_id_key_map.get(v)),
        );
        if let Some(n_effect) = N_EFFECT_MAP.get(&a_effect.id) {
            self.charge = n_effect
                .charge
                .as_ref()
                .and_then(|n_charge| REffectCharge::try_from_n_charge(n_charge, attr_id_key_map));
            self.projectee_filter = n_effect.projectee_filter.as_ref().and_then(|n_projectee_filter| {
                REffectProjecteeFilter::try_from_n_projectee_filter(
                    n_projectee_filter,
                    item_list_id_key_map,
                    attr_id_key_map,
                )
            });
            if let Some(modifier_proj_attrs_getter) = n_effect.modifier_proj_attrs_getter {
                let proj_attr_ids = modifier_proj_attrs_getter(a_effect);
                self.modifier_proj_attr_keys =
                    proj_attr_ids.map(|opt| opt.and_then(|attr_id| attr_id_key_map.get(&attr_id).copied()));
            }
        }
        // Data derived from key-dependent data
        self.is_active_with_duration = self.state == AState::Active && self.duration_attr_key.is_some();
    }
    pub(crate) fn is_active(&self) -> bool {
        self.state == AState::Active
    }
    // Misc methods
    pub(crate) fn activates_charge(&self) -> bool {
        let charge_info = match &self.charge {
            Some(charge_info) => charge_info,
            None => return false,
        };
        if !charge_info.activates_charge {
            return false;
        }
        matches!(charge_info.location, REffectChargeLoc::Loaded(_))
    }
    pub(crate) fn activates_charge_for_item(&self, item: &RItem) -> bool {
        if !self.activates_charge() {
            return false;
        }
        // Only default effects can activate regular charge
        let defeff_key = match item.defeff_key {
            Some(defeff_key) => defeff_key,
            None => return false,
        };
        defeff_key == self.key
    }
    pub(crate) fn activates_autocharge(&self) -> bool {
        let charge_info = match &self.charge {
            Some(charge_info) => charge_info,
            None => return false,
        };
        if !charge_info.activates_charge {
            return false;
        }
        matches!(charge_info.location, REffectChargeLoc::Autocharge(_))
    }
}
