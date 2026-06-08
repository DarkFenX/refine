use crate::{
    ad::{AAttrId, ABuffId, AEffect, AEffectCatId, AEffectId, AItemListId},
    nd::{
        N_EFFECT_MAP, NEffectBreacherOutputGetter, NEffectDmgKindGetter, NEffectDmgOutputGetter,
        NEffectGeneralOutputGetter,
    },
    rd::{
        RAttrId, RBuffId, REffectBuff, REffectCharge, REffectChargeLoc, REffectEcm, REffectId, REffectLocalOpcSpec,
        REffectMining, REffectModifier, REffectNeut, REffectProjModSpec, REffectProjOpcSpec, REffectProjecteeFilter,
        REffectSpoolAttrs, RItem, RItemListId, RState,
    },
    svc::calc::CalcCustomModifier,
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
    pub(crate) cloaks_carrier: bool,
    pub(crate) kills_item: bool,
    pub(crate) is_active_with_duration: bool,
    pub(crate) calc_custom_mod: Option<CalcCustomModifier>,
    // References to attributes which are used to describe some effect properties
    pub(crate) discharge_attr_rid: Option<RAttrId>,
    pub(crate) duration_attr_rid: Option<RAttrId>,
    pub(crate) range_attr_rid: Option<RAttrId>,
    pub(crate) falloff_attr_rid: Option<RAttrId>,
    pub(crate) track_attr_rid: Option<RAttrId>,
    pub(crate) chance_attr_rid: Option<RAttrId>,
    pub(crate) resist_attr_rid: Option<RAttrId>,
    pub(crate) spool_attr_rids: Option<REffectSpoolAttrs>,
    pub(crate) proj_mod: Option<REffectProjModSpec>,
    // Output getters/specs
    pub(crate) dmg_kind: Option<NEffectDmgKindGetter>,
    pub(crate) normal_dmg: Option<REffectProjOpcSpec<NEffectDmgOutputGetter>>,
    pub(crate) breacher_dmg: Option<REffectProjOpcSpec<NEffectBreacherOutputGetter>>,
    pub(crate) mining_ore: Option<REffectMining>,
    pub(crate) mining_ice: Option<REffectMining>,
    pub(crate) mining_gas: Option<REffectMining>,
    pub(crate) outgoing_shield_rep: Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) outgoing_armor_rep: Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) outgoing_hull_rep: Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) local_shield_rep: Option<REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) local_armor_rep: Option<REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) local_hull_rep: Option<REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) cap_consume: Option<REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) neut: Option<REffectNeut>,
    pub(crate) nosf: Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) outgoing_cap: Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) cap_inject: Option<REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
    pub(crate) ecm: Option<REffectEcm>,
}
impl REffect {
    pub(crate) fn is_active(&self) -> bool {
        self.state == RState::Active
    }
    pub(crate) fn activates_charge(&self) -> bool {
        let Some(charge_info) = &self.charge else {
            return false;
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
        let Some(defeff_rid) = item.defeff_rid else {
            return false;
        };
        defeff_rid == self.rid
    }
    pub(crate) fn activates_autocharge(&self) -> bool {
        let Some(charge_info) = &self.charge else {
            return false;
        };
        if !charge_info.activates_charge {
            return false;
        }
        matches!(charge_info.location, REffectChargeLoc::Autocharge(_))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
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
            cloaks_carrier: n_effect.map(|n| n.cloaks_carrier).unwrap_or(false),
            kills_item: n_effect.map(|n| n.kills_item).unwrap_or(false),
            calc_custom_mod: n_effect.and_then(|n| n.calc_custom_mod),
            dmg_kind: n_effect.and_then(|n| n.dmg_kind),
            // Fields which depend on data not available during instantiation
            modifiers: Default::default(),
            stopped_effect_rids: Default::default(),
            buff: Default::default(),
            charge: Default::default(),
            projectee_filter: Default::default(),
            spool_attr_rids: Default::default(),
            discharge_attr_rid: Default::default(),
            duration_attr_rid: Default::default(),
            range_attr_rid: Default::default(),
            falloff_attr_rid: Default::default(),
            track_attr_rid: Default::default(),
            chance_attr_rid: Default::default(),
            resist_attr_rid: Default::default(),
            is_active_with_duration: Default::default(),
            proj_mod: Default::default(),
            normal_dmg: Default::default(),
            breacher_dmg: Default::default(),
            mining_ore: Default::default(),
            mining_ice: Default::default(),
            mining_gas: Default::default(),
            outgoing_shield_rep: Default::default(),
            outgoing_armor_rep: Default::default(),
            outgoing_hull_rep: Default::default(),
            local_shield_rep: Default::default(),
            local_armor_rep: Default::default(),
            local_hull_rep: Default::default(),
            cap_consume: Default::default(),
            neut: Default::default(),
            nosf: Default::default(),
            outgoing_cap: Default::default(),
            cap_inject: Default::default(),
            ecm: Default::default(),
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
                .and_then(|n_spool_attrs| REffectSpoolAttrs::try_from_n_spool_attrs(n_spool_attrs, attr_aid_rid_map));
            if let Some(mspec) = &n_effect.proj_mod {
                self.proj_mod = Some(REffectProjModSpec::from_n_proj_mod_spec(
                    mspec,
                    a_effect,
                    attr_aid_rid_map,
                ));
            }
            self.normal_dmg = n_effect
                .normal_dmg
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.breacher_dmg = n_effect
                .breacher_dmg
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.mining_ore = n_effect
                .mining_ore
                .as_ref()
                .map(|mining| REffectMining::from_n_effect_mining(mining, attr_aid_rid_map));
            self.mining_ice = n_effect
                .mining_ice
                .as_ref()
                .map(|mining| REffectMining::from_n_effect_mining(mining, attr_aid_rid_map));
            self.mining_gas = n_effect
                .mining_gas
                .as_ref()
                .map(|mining| REffectMining::from_n_effect_mining(mining, attr_aid_rid_map));
            self.outgoing_shield_rep = n_effect
                .outgoing_shield_rep
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.outgoing_armor_rep = n_effect
                .outgoing_armor_rep
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.outgoing_hull_rep = n_effect
                .outgoing_hull_rep
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.local_shield_rep = n_effect
                .local_shield_rep
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.local_armor_rep = n_effect
                .local_armor_rep
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.local_hull_rep = n_effect
                .local_hull_rep
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.cap_consume = n_effect
                .cap_consume
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.neut = n_effect
                .neut
                .as_ref()
                .map(|neut| REffectNeut::from_n_effect_neut(neut, attr_aid_rid_map));
            self.nosf = n_effect
                .nosf
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.outgoing_cap = n_effect
                .outgoing_cap
                .as_ref()
                .map(|ospec| REffectProjOpcSpec::from_n_proj_opc_spec(ospec, attr_aid_rid_map));
            self.cap_inject = n_effect
                .cap_inject
                .as_ref()
                .map(|ospec| REffectLocalOpcSpec::from_n_local_opc_spec(ospec, attr_aid_rid_map));
            self.ecm = n_effect
                .ecm
                .as_ref()
                .map(|ecm| REffectEcm::from_n_effect_ecm(ecm, attr_aid_rid_map));
        }
        // Generate default cap consumption OPC spec here, since it's not defined on NEffects for
        // all effects which need it.
        if self.cap_consume.is_none() && self.discharge_attr_rid.is_some() {
            self.cap_consume = Some(REffectLocalOpcSpec {
                base: NEffectGeneralOutputGetter::CapConsumer,
                charge_mult: None,
                limit_attr_rid: None,
            })
        }
        self.is_active_with_duration = self.state == RState::Active && self.duration_attr_rid.is_some();
    }
}
