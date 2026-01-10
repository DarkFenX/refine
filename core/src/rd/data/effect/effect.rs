use crate::{
    ad::{AAttrId, ABuffId, AEffect, AEffectCatId, AEffectId, AItemListId},
    misc::{DmgKinds, Ecm, MiningAmount},
    nd::{N_EFFECT_MAP, NBreacherDmgGetter, NCalcCustomizer, NDmgKindGetter, NProjMultGetter},
    num::{PValue, Value},
    rd::{
        RAttrId, RBuffId, REffectBuff, REffectCharge, REffectChargeLoc, REffectId, REffectLocalOpcSpec,
        REffectModifier, REffectProjOpcSpec, REffectProjecteeFilter, RItem, RItemListId, RSpoolAttrs, RState,
    },
    util::RMap,
};

// Represents an effect.
//
// Effects are higher-level modification descriptors, as opposed to modifiers, which are
// lower-level. An effect can contain any number of modifiers under a single roof, accompanied by
// extra effect-wide properties.
pub(crate) struct REffect {
    pub(crate) aid: AEffectId,
    pub(crate) rid: REffectId,
    pub(crate) category: AEffectCatId,
    pub(crate) state: RState,
    pub(crate) charge: Option<REffectCharge>,
    pub(crate) buff: Option<REffectBuff>,
    pub(crate) projectee_filter: Option<REffectProjecteeFilter>,
    pub(crate) modifiers: Vec<REffectModifier>,
    pub(crate) stopped_effect_rids: Vec<REffectId>,
    pub(crate) is_assist: bool,
    pub(crate) is_offense: bool,
    pub(crate) banned_in_hisec: bool,
    pub(crate) banned_in_lowsec: bool,
    pub(crate) ignore_offmod_immunity: bool,
    pub(crate) kills_item: bool,
    pub(crate) is_active_with_duration: bool,
    pub(crate) calc_customizer: Option<NCalcCustomizer>,
    // References to attributes which are used to describe some effect properties
    pub(crate) discharge_attr_rid: Option<RAttrId>,
    pub(crate) duration_attr_rid: Option<RAttrId>,
    pub(crate) range_attr_rid: Option<RAttrId>,
    pub(crate) falloff_attr_rid: Option<RAttrId>,
    pub(crate) track_attr_rid: Option<RAttrId>,
    pub(crate) chance_attr_rid: Option<RAttrId>,
    pub(crate) resist_attr_rid: Option<RAttrId>,
    pub(crate) spool_attr_rids: Option<RSpoolAttrs>,
    pub(crate) modifier_proj_attr_rids: [Option<RAttrId>; 2],
    pub(crate) modifier_proj_mult_getter: Option<NProjMultGetter>,
    // Output getters/specs
    pub(crate) dmg_kind_getter: Option<NDmgKindGetter>,
    pub(crate) normal_dmg_opc_spec: Option<REffectProjOpcSpec<DmgKinds<PValue>>>,
    pub(crate) breacher_dmg_opc_getter: Option<NBreacherDmgGetter>,
    pub(crate) mining_ore_opc_spec: Option<REffectProjOpcSpec<MiningAmount>>,
    pub(crate) mining_ice_opc_spec: Option<REffectProjOpcSpec<MiningAmount>>,
    pub(crate) mining_gas_opc_spec: Option<REffectProjOpcSpec<MiningAmount>>,
    pub(crate) outgoing_shield_rep_opc_spec: Option<REffectProjOpcSpec<PValue>>,
    pub(crate) outgoing_armor_rep_opc_spec: Option<REffectProjOpcSpec<PValue>>,
    pub(crate) outgoing_hull_rep_opc_spec: Option<REffectProjOpcSpec<PValue>>,
    pub(crate) local_shield_rep_opc_spec: Option<REffectLocalOpcSpec<PValue>>,
    pub(crate) local_armor_rep_opc_spec: Option<REffectLocalOpcSpec<PValue>>,
    pub(crate) local_hull_rep_opc_spec: Option<REffectLocalOpcSpec<PValue>>,
    pub(crate) neut_opc_spec: Option<REffectProjOpcSpec<PValue>>,
    pub(crate) outgoing_cap_opc_spec: Option<REffectProjOpcSpec<PValue>>,
    pub(crate) cap_inject_opc_spec: Option<REffectLocalOpcSpec<PValue>>,
    pub(crate) ecm_opc_spec: Option<REffectProjOpcSpec<Ecm>>,
}
impl REffect {
    pub(in crate::rd) fn from_a_effect(effect_rid: REffectId, a_effect: &AEffect) -> Self {
        let n_effect = N_EFFECT_MAP.get(&a_effect.id);
        let state = RState::from_a_state(&a_effect.state);
        Self {
            aid: a_effect.id,
            rid: effect_rid,
            category: a_effect.category,
            state,
            is_assist: a_effect.is_assist && state == RState::Active,
            is_offense: a_effect.is_offense && state == RState::Active,
            banned_in_hisec: a_effect.banned_in_hisec && state == RState::Active,
            banned_in_lowsec: a_effect.banned_in_lowsec && state == RState::Active,
            ignore_offmod_immunity: n_effect.map(|n| n.ignore_offmod_immunity).unwrap_or(false),
            kills_item: n_effect.map(|n| n.kills_item).unwrap_or(false),
            calc_customizer: n_effect.and_then(|n| n.calc_customizer),
            modifier_proj_mult_getter: n_effect.and_then(|n| n.modifier_proj_mult_getter),
            dmg_kind_getter: n_effect.and_then(|n| n.dmg_kind_getter),
            breacher_dmg_opc_getter: n_effect.and_then(|n| n.breacher_dmg_opc_getter),
            // Fields which depend on data not available during instantiation
            modifiers: Default::default(),
            stopped_effect_rids: Default::default(),
            buff: Default::default(),
            charge: Default::default(),
            projectee_filter: Default::default(),
            spool_attr_rids: Default::default(),
            modifier_proj_attr_rids: Default::default(),
            discharge_attr_rid: Default::default(),
            duration_attr_rid: Default::default(),
            range_attr_rid: Default::default(),
            falloff_attr_rid: Default::default(),
            track_attr_rid: Default::default(),
            chance_attr_rid: Default::default(),
            resist_attr_rid: Default::default(),
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
    pub(in crate::rd) fn fill_runtime(
        &mut self,
        a_effects: &RMap<AEffectId, AEffect>,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        effect_aid_rid_map: &RMap<AEffectId, REffectId>,
        buff_aid_rid_map: &RMap<ABuffId, RBuffId>,
    ) {
        let a_effect = a_effects.get(&self.aid).unwrap();
        self.buff = a_effect.buff.as_ref().and_then(|a_effect_buff| {
            REffectBuff::try_from_a_buff(a_effect_buff, item_list_aid_rid_map, attr_aid_rid_map, buff_aid_rid_map)
        });
        self.discharge_attr_rid = a_effect
            .discharge_attr_id
            .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid))
            .copied();
        self.duration_attr_rid = a_effect
            .duration_attr_id
            .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid))
            .copied();
        self.range_attr_rid = a_effect.range_attr_id.and_then(|id| attr_aid_rid_map.get(&id)).copied();
        self.falloff_attr_rid = a_effect
            .falloff_attr_id
            .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid))
            .copied();
        self.falloff_attr_rid = a_effect
            .falloff_attr_id
            .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid))
            .copied();
        self.track_attr_rid = a_effect.track_attr_id.and_then(|id| attr_aid_rid_map.get(&id)).copied();
        self.chance_attr_rid = a_effect
            .chance_attr_id
            .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid))
            .copied();
        self.resist_attr_rid = a_effect
            .resist_attr_id
            .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid))
            .copied();
        self.modifiers.extend(
            a_effect
                .modifiers
                .iter()
                .filter_map(|a_effect_mod| REffectModifier::try_from_a_effect_mod(a_effect_mod, attr_aid_rid_map)),
        );
        self.stopped_effect_rids.extend(
            a_effect
                .stopped_effect_ids
                .iter()
                .filter_map(|effect_aid| effect_aid_rid_map.get(effect_aid)),
        );
        if let Some(n_effect) = N_EFFECT_MAP.get(&a_effect.id) {
            self.charge = n_effect
                .charge
                .as_ref()
                .and_then(|n_charge| REffectCharge::try_from_n_charge(n_charge, attr_aid_rid_map));
            self.projectee_filter = n_effect.projectee_filter.as_ref().and_then(|n_projectee_filter| {
                REffectProjecteeFilter::try_from_n_projectee_filter(
                    n_projectee_filter,
                    item_list_aid_rid_map,
                    attr_aid_rid_map,
                )
            });
            self.spool_attr_rids = n_effect
                .spool_attrs
                .as_ref()
                .and_then(|n_spool_attrs| RSpoolAttrs::try_from_n_spool_attrs(n_spool_attrs, attr_aid_rid_map));
            if let Some(modifier_proj_attrs_getter) = n_effect.modifier_proj_attrs_getter {
                let proj_attr_aids = modifier_proj_attrs_getter(a_effect);
                self.modifier_proj_attr_rids = proj_attr_aids
                    .map(|attr_aid| attr_aid.and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid).copied()));
            }
            self.normal_dmg_opc_spec = n_effect
                .normal_dmg_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.mining_ore_opc_spec = n_effect
                .mining_ore_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.mining_ice_opc_spec = n_effect
                .mining_ice_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.mining_gas_opc_spec = n_effect
                .mining_gas_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.outgoing_shield_rep_opc_spec = n_effect
                .outgoing_shield_rep_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.outgoing_armor_rep_opc_spec = n_effect
                .outgoing_armor_rep_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.outgoing_hull_rep_opc_spec = n_effect
                .outgoing_hull_rep_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.local_shield_rep_opc_spec = n_effect
                .local_shield_rep_opc_spec
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.local_armor_rep_opc_spec = n_effect
                .local_armor_rep_opc_spec
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.local_hull_rep_opc_spec = n_effect
                .local_hull_rep_opc_spec
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.neut_opc_spec = n_effect
                .neut_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.outgoing_cap_opc_spec = n_effect
                .outgoing_cap_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.cap_inject_opc_spec = n_effect
                .cap_inject_opc_spec
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.ecm_opc_spec = n_effect
                .ecm_opc_spec
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
        }
        self.is_active_with_duration = self.state == RState::Active && self.duration_attr_rid.is_some();
    }
    pub(crate) fn is_active(&self) -> bool {
        self.state == RState::Active
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
        let defeff_rid = match item.defeff_rid {
            Some(defeff_rid) => defeff_rid,
            None => return false,
        };
        defeff_rid == self.rid
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
