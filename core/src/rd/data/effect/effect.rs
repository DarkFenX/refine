use crate::{
    ad::{AAttrId, ABuffId, AEffect, AEffectCatId, AEffectId, AItemListId, AState},
    def::AttrVal,
    misc::{DmgKinds, Ecm, MiningAmount},
    nd::{N_EFFECT_MAP, NBreacherDmgGetter, NCalcCustomizer, NDmgKindGetter, NProjMultGetter},
    rd::{
        RAttrId, RBuffId, REffectBuff, REffectCharge, REffectChargeLoc, REffectId, REffectLocalOpcSpec,
        REffectModifier, REffectProjOpcSpec, REffectProjecteeFilter, RItem, RItemListId, RSpoolAttrs,
    },
    util::RMap,
};

// Represents an effect.
//
// Effects are higher-level modification descriptors, as opposed to modifiers, which are
// lower-level. An effect can contain any number of modifiers under a single roof, accompanied by
// extra effect-wide properties.
pub(crate) struct REffect {
    pub(crate) key: REffectId,
    pub(crate) id: AEffectId,
    pub(crate) category: AEffectCatId,
    pub(crate) state: AState,
    pub(crate) is_assist: bool,
    pub(crate) is_offense: bool,
    pub(crate) banned_in_hisec: bool,
    pub(crate) banned_in_lowsec: bool,
    pub(crate) ignore_offmod_immunity: bool,
    pub(crate) kills_item: bool,
    pub(crate) calc_customizer: Option<NCalcCustomizer>,
    pub(crate) modifier_proj_mult_getter: Option<NProjMultGetter>,
    // Output getters
    pub(crate) dmg_kind_getter: Option<NDmgKindGetter>,
    pub(crate) breacher_dmg_opc_getter: Option<NBreacherDmgGetter>,
    // Fields which depend on slab keys
    pub(crate) modifiers: Vec<REffectModifier>,
    pub(crate) stopped_effect_keys: Vec<REffectId>,
    pub(crate) buff: Option<REffectBuff>,
    pub(crate) charge: Option<REffectCharge>,
    pub(crate) projectee_filter: Option<REffectProjecteeFilter>,
    pub(crate) spool_attr_keys: Option<RSpoolAttrs>,
    pub(crate) modifier_proj_attr_keys: [Option<RAttrId>; 2],
    pub(crate) discharge_attr_key: Option<RAttrId>,
    pub(crate) duration_attr_key: Option<RAttrId>,
    pub(crate) range_attr_key: Option<RAttrId>,
    pub(crate) falloff_attr_key: Option<RAttrId>,
    pub(crate) track_attr_key: Option<RAttrId>,
    pub(crate) chance_attr_key: Option<RAttrId>,
    pub(crate) resist_attr_key: Option<RAttrId>,
    pub(crate) is_active_with_duration: bool,
    // Output specs depend on slab keys as well
    pub(crate) normal_dmg_opc_spec: Option<REffectProjOpcSpec<DmgKinds<AttrVal>>>,
    pub(crate) mining_ore_opc_spec: Option<REffectProjOpcSpec<MiningAmount>>,
    pub(crate) mining_ice_opc_spec: Option<REffectProjOpcSpec<MiningAmount>>,
    pub(crate) mining_gas_opc_spec: Option<REffectProjOpcSpec<MiningAmount>>,
    pub(crate) outgoing_shield_rep_opc_spec: Option<REffectProjOpcSpec<AttrVal>>,
    pub(crate) outgoing_armor_rep_opc_spec: Option<REffectProjOpcSpec<AttrVal>>,
    pub(crate) outgoing_hull_rep_opc_spec: Option<REffectProjOpcSpec<AttrVal>>,
    pub(crate) local_shield_rep_opc_spec: Option<REffectLocalOpcSpec<AttrVal>>,
    pub(crate) local_armor_rep_opc_spec: Option<REffectLocalOpcSpec<AttrVal>>,
    pub(crate) local_hull_rep_opc_spec: Option<REffectLocalOpcSpec<AttrVal>>,
    pub(crate) neut_opc_spec: Option<REffectProjOpcSpec<AttrVal>>,
    pub(crate) outgoing_cap_opc_spec: Option<REffectProjOpcSpec<AttrVal>>,
    pub(crate) cap_inject_opc_spec: Option<REffectLocalOpcSpec<AttrVal>>,
    pub(crate) ecm_opc_spec: Option<REffectProjOpcSpec<Ecm>>,
}
impl REffect {
    pub(in crate::rd) fn from_a_effect(effect_key: REffectId, a_effect: &AEffect) -> Self {
        let n_effect = N_EFFECT_MAP.get(&a_effect.id);
        Self {
            key: effect_key,
            id: a_effect.id,
            category: a_effect.category,
            state: a_effect.state,
            is_assist: a_effect.is_assist && a_effect.state == AState::Active,
            is_offense: a_effect.is_offense && a_effect.state == AState::Active,
            banned_in_hisec: a_effect.banned_in_hisec && a_effect.state == AState::Active,
            banned_in_lowsec: a_effect.banned_in_lowsec && a_effect.state == AState::Active,
            ignore_offmod_immunity: n_effect.map(|n| n.ignore_offmod_immunity).unwrap_or(false),
            kills_item: n_effect.map(|n| n.kills_item).unwrap_or(false),
            calc_customizer: n_effect.and_then(|n| n.calc_customizer),
            modifier_proj_mult_getter: n_effect.and_then(|n| n.modifier_proj_mult_getter),
            // Output getters
            dmg_kind_getter: n_effect.and_then(|n| n.dmg_kind_getter),
            breacher_dmg_opc_getter: n_effect.and_then(|n| n.breacher_dmg_opc_getter),
            // Fields which depend on slab keys
            modifiers: Default::default(),
            stopped_effect_keys: Default::default(),
            buff: Default::default(),
            charge: Default::default(),
            projectee_filter: Default::default(),
            spool_attr_keys: Default::default(),
            modifier_proj_attr_keys: Default::default(),
            discharge_attr_key: Default::default(),
            duration_attr_key: Default::default(),
            range_attr_key: Default::default(),
            falloff_attr_key: Default::default(),
            track_attr_key: Default::default(),
            chance_attr_key: Default::default(),
            resist_attr_key: Default::default(),
            is_active_with_duration: Default::default(),
            normal_dmg_opc_spec: Default::default(),
            mining_ore_opc_spec: Default::default(),
            mining_ice_opc_spec: Default::default(),
            mining_gas_opc_spec: Default::default(),
            outgoing_shield_rep_opc_spec: Default::default(),
            outgoing_armor_rep_opc_spec: Default::default(),
            outgoing_hull_rep_opc_spec: Default::default(),
            local_shield_rep_opc_spec: Default::default(),
            local_armor_rep_opc_spec: Default::default(),
            local_hull_rep_opc_spec: Default::default(),
            neut_opc_spec: Default::default(),
            outgoing_cap_opc_spec: Default::default(),
            cap_inject_opc_spec: Default::default(),
            ecm_opc_spec: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_key_dependents(
        &mut self,
        a_effects: &RMap<AEffectId, AEffect>,
        item_list_id_key_map: &RMap<AItemListId, RItemListId>,
        attr_id_key_map: &RMap<AAttrId, RAttrId>,
        effect_id_key_map: &RMap<AEffectId, REffectId>,
        buff_id_key_map: &RMap<ABuffId, RBuffId>,
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
                .stopped_effect_ids
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
            self.spool_attr_keys = n_effect
                .spool_attr_ids
                .as_ref()
                .and_then(|n_spool_attrs| RSpoolAttrs::try_from_n_spool_attrs(n_spool_attrs, attr_id_key_map));
            if let Some(modifier_proj_attrs_getter) = n_effect.modifier_proj_attrs_getter {
                let proj_attr_ids = modifier_proj_attrs_getter(a_effect);
                self.modifier_proj_attr_keys =
                    proj_attr_ids.map(|opt| opt.and_then(|attr_id| attr_id_key_map.get(&attr_id).copied()));
            }
            self.normal_dmg_opc_spec = n_effect
                .normal_dmg_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.mining_ore_opc_spec = n_effect
                .mining_ore_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.mining_ice_opc_spec = n_effect
                .mining_ice_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.mining_gas_opc_spec = n_effect
                .mining_gas_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.outgoing_shield_rep_opc_spec = n_effect
                .outgoing_shield_rep_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.outgoing_armor_rep_opc_spec = n_effect
                .outgoing_armor_rep_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.outgoing_hull_rep_opc_spec = n_effect
                .outgoing_hull_rep_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.local_shield_rep_opc_spec = n_effect
                .local_shield_rep_opc_spec
                .as_ref()
                .map(|v| REffectLocalOpcSpec::from_n_local_opc_spec(v, attr_id_key_map));
            self.local_armor_rep_opc_spec = n_effect
                .local_armor_rep_opc_spec
                .as_ref()
                .map(|v| REffectLocalOpcSpec::from_n_local_opc_spec(v, attr_id_key_map));
            self.local_hull_rep_opc_spec = n_effect
                .local_hull_rep_opc_spec
                .as_ref()
                .map(|v| REffectLocalOpcSpec::from_n_local_opc_spec(v, attr_id_key_map));
            self.neut_opc_spec = n_effect
                .neut_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.outgoing_cap_opc_spec = n_effect
                .outgoing_cap_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
            self.cap_inject_opc_spec = n_effect
                .cap_inject_opc_spec
                .as_ref()
                .map(|v| REffectLocalOpcSpec::from_n_local_opc_spec(v, attr_id_key_map));
            self.ecm_opc_spec = n_effect
                .ecm_opc_spec
                .as_ref()
                .map(|v| REffectProjOpcSpec::from_n_proj_opc_spec(v, attr_id_key_map));
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
