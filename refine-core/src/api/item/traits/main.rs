use itertools::Itertools;

use crate::{
    AttrId, Count, CtlAffectors, DpsProfile, EffectId, EffectMode, FitId, ItemAttrValues, ItemEffectInfo, ItemId,
    ItemTypeId, Modification, OptionalReload, PValue, UnitInterval, Value,
    api::{AffectionDir, ItemSealed, active_stat_prepare},
    err::{
        ItemAttrGetError, ItemAttrsIterError, ItemEffectsIterError, ItemModifiersIterError,
        basic::{AttrFoundError, ItemLoadedError},
    },
    stats::{
        StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger, StatCritOptions, StatDmg, StatDmgApplied, StatEhp, StatErps,
        StatHp, StatInJam, StatItemChargeOptions, StatItemStateOptions, StatJump, StatJumpRange, StatMining,
        StatMiningResourceKind, StatOutReps, StatResists, StatRps, StatSensors, StatTimeOptions,
        err::{
            StatAgilityError, StatItemAppliedError, StatItemError, StatJumpError, StatMaxWarpRangeError,
            StatProbingSizeError, StatWarpSpeedError,
        },
    },
    svc::{
        cycle::CseqMap,
        vast::{StatCapBlcSrcKindsInt, StatCapSimStaggerInt},
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Methods shared by every item kind.
#[expect(private_bounds)]
pub trait ItemCommon: ItemSealed {
    fn get_item_id(&self) -> ItemId {
        self.get_core().u_data.items.ext_id_by_int_id(self.get_uid())
    }
    fn get_type_id(&self) -> ItemTypeId {
        let type_aid = self.get_core().u_data.items.get(self.get_uid()).get_rendered_type_aid();
        ItemTypeId::from_aid(type_aid)
    }
    fn iter_effects(&self) -> Result<impl ExactSizeIterator<Item = (EffectId, ItemEffectInfo)>, ItemEffectsIterError> {
        let core = self.get_core();
        let item_uid = self.get_uid();
        let item = core.u_data.items.get(item_uid);
        let (effect_rids, reffs) = match (item.get_r_item_base(), item.get_reffs()) {
            (Some(item_rib), Some(reffs)) => (item_rib.effects.keys(), reffs),
            _ => {
                return Err(ItemLoadedError {
                    item_id: core.u_data.items.ext_id_by_int_id(item_uid),
                }
                .into());
            }
        };
        let effect_infos = effect_rids.map(move |&effect_rid| {
            let effect_aid = core.u_data.r_data.get_effect_by_rid(effect_rid).aid;
            let running = reffs.contains(&effect_rid);
            let mode = item.get_effect_mode_by_rid(&effect_rid);
            (EffectId::from_aid(effect_aid), ItemEffectInfo { running, mode })
        });
        Ok(effect_infos)
    }
    fn iter_effect_mode_overrides(&self) -> impl ExactSizeIterator<Item = (EffectId, EffectMode)> {
        let core = self.get_core();
        let item_uid = self.get_uid();
        let item = core.u_data.items.get(item_uid);
        item.iter_effect_mode_overrides()
            .map(|(effect_aid, effect_mode)| (EffectId::from_aid(effect_aid), effect_mode))
    }
    fn get_attr(&mut self, attr_id: &AttrId) -> Result<ItemAttrValues, ItemAttrGetError> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let attr_aid = attr_id.into_aid();
        let Some(attr_rid) = core.u_data.r_data.get_attr_rid_by_aid(&attr_aid) else {
            return Err(AttrFoundError { attr_id: *attr_id }.into());
        };
        match core.internal_get_item_attr(item_uid, attr_rid) {
            Ok(calc_vals) => Ok(ItemAttrValues::from_calc_attr_vals(calc_vals)),
            Err(error) => Err(ItemLoadedError {
                item_id: self.get_core_mut().u_data.items.ext_id_by_int_id(error.item_uid),
            }
            .into()),
        }
    }
    fn iter_attrs(&mut self) -> Result<impl ExactSizeIterator<Item = (AttrId, ItemAttrValues)>, ItemAttrsIterError> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        match core.svc.iter_item_attr_vals(&core.u_data, item_uid) {
            Ok(attr_iter) => Ok(attr_iter.map(|(attr_rid, calc_vals)| {
                (
                    AttrId::from_aid(core.u_data.r_data.get_attr_by_rid(attr_rid).aid),
                    ItemAttrValues::from_calc_attr_vals(calc_vals),
                )
            })),
            Err(error) => Err(ItemLoadedError {
                item_id: core.u_data.items.ext_id_by_int_id(error.item_uid),
            }
            .into()),
        }
    }
    fn iter_modifiers(
        &mut self,
    ) -> Result<
        impl ExactSizeIterator<Item = (AttrId, impl ExactSizeIterator<Item = Modification>)>,
        ItemModifiersIterError,
    > {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        match core.svc.iter_item_mods(&core.u_data, item_uid) {
            Ok(mods_iter) => Ok(mods_iter.into_iter().map(|(attr_rid, modifications)| {
                (
                    AttrId::from_aid(core.u_data.r_data.get_attr_by_rid(attr_rid).aid),
                    modifications
                        .into_iter()
                        .map(|calc_mod_info| Modification::from_calc(calc_mod_info, &core.u_data)),
                )
            })),
            Err(err) => Err(ItemLoadedError {
                item_id: core.u_data.items.ext_id_by_int_id(err.item_uid),
            }
            .into()),
        }
    }
    fn set_effect_mode(&mut self, effect_id: &EffectId, effect_mode: EffectMode) {
        let item_uid = self.get_uid();
        self.get_core_mut()
            .internal_set_effect_id_mode(item_uid, effect_id.into_aid(), effect_mode);
    }
    fn set_effect_modes(&mut self, effect_modes: impl IntoIterator<Item = (EffectId, EffectMode)>) {
        let item_uid = self.get_uid();
        self.get_core_mut().internal_set_effect_id_modes(
            item_uid,
            effect_modes
                .into_iter()
                .map(|(effect_id, effect_mode)| (effect_id.into_aid(), effect_mode)),
        );
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - output
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_dmg(
        &mut self,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
        state_options: StatItemStateOptions,
    ) -> Result<StatDmg, StatItemError<!>> {
        let savepoint = active_stat_prepare(self, charge_options, state_options);
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_dmg_raw(
                &mut CseqMap::new(),
                &core.u_data,
                item_uid,
                time_options,
                crit_options,
                charge_options,
            )
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_dmg_applied(
        &mut self,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
        state_options: StatItemStateOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        // Check projectee before switching states, so that its errors leave states untouched
        let projectee_uid = self.get_core_mut().u_data.get_projectee_uid(projectee_item_id)?;
        let savepoint = active_stat_prepare(self, charge_options, state_options);
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_dmg_applied(
                &mut CseqMap::new(),
                &core.u_data,
                item_uid,
                time_options,
                crit_options,
                charge_options,
                projectee_uid,
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_mps(
        &mut self,
        time_options: StatTimeOptions,
        resource_kind: StatMiningResourceKind,
        state_options: StatItemStateOptions,
    ) -> Result<StatMining, StatItemError<!>> {
        let savepoint = active_stat_prepare(self, StatItemChargeOptions::Exclude, state_options);
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_mps(&mut CseqMap::new(), &core.u_data, item_uid, time_options, resource_kind)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_outgoing_nps(
        &mut self,
        time_options: StatTimeOptions,
        charge_options: StatItemChargeOptions,
        state_options: StatItemStateOptions,
    ) -> Result<PValue, StatItemError<!>> {
        let savepoint = active_stat_prepare(self, charge_options, state_options);
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_outgoing_nps(
                &mut CseqMap::new(),
                &core.u_data,
                item_uid,
                time_options,
                charge_options,
                None,
            )
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_outgoing_nps_applied(
        &mut self,
        time_options: StatTimeOptions,
        charge_options: StatItemChargeOptions,
        state_options: StatItemStateOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        // Check projectee before switching states, so that its errors leave states untouched
        let projectee_uid = self.get_core_mut().u_data.get_projectee_uid(projectee_item_id)?;
        let savepoint = active_stat_prepare(self, charge_options, state_options);
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_outgoing_nps(
                &mut CseqMap::new(),
                &core.u_data,
                item_uid,
                time_options,
                charge_options,
                Some(projectee_uid),
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_outgoing_rps(
        &mut self,
        time_options: StatTimeOptions,
        state_options: StatItemStateOptions,
    ) -> Result<StatOutReps, StatItemError<!>> {
        let savepoint = active_stat_prepare(self, StatItemChargeOptions::Exclude, state_options);
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_outgoing_rps(&mut CseqMap::new(), &core.u_data, item_uid, time_options, None)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_outgoing_rps_applied(
        &mut self,
        time_options: StatTimeOptions,
        state_options: StatItemStateOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatOutReps, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        // Check projectee before switching states, so that its errors leave states untouched
        let projectee_uid = self.get_core_mut().u_data.get_projectee_uid(projectee_item_id)?;
        let savepoint = active_stat_prepare(self, StatItemChargeOptions::Exclude, state_options);
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_outgoing_rps(
                &mut CseqMap::new(),
                &core.u_data,
                item_uid,
                time_options,
                Some(projectee_uid),
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_outgoing_cps(
        &mut self,
        time_options: StatTimeOptions,
        state_options: StatItemStateOptions,
    ) -> Result<PValue, StatItemError<!>> {
        let savepoint = active_stat_prepare(self, StatItemChargeOptions::Exclude, state_options);
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_outgoing_cps(&mut CseqMap::new(), &core.u_data, item_uid, time_options, None)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        state_options: StatItemStateOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        // Check projectee before switching states, so that its errors leave states untouched
        let projectee_uid = self.get_core_mut().u_data.get_projectee_uid(projectee_item_id)?;
        let savepoint = active_stat_prepare(self, StatItemChargeOptions::Exclude, state_options);
        let core = self.get_core_mut();
        let result = core
            .svc
            .get_stat_item_outgoing_cps(
                &mut CseqMap::new(),
                &core.u_data,
                item_uid,
                time_options,
                Some(projectee_uid),
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - tank
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_resists(&mut self) -> Result<StatResists, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_resists(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_hp(&mut self) -> Result<StatHp, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_hp(&mut CseqMap::new(), &core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_ehp(&mut self, incoming_dps: Option<DpsProfile>) -> Result<StatEhp, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_ehp(&mut CseqMap::new(), &core.u_data, item_uid, incoming_dps)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_wc_ehp(&mut self) -> Result<StatEhp, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_wc_ehp(&mut CseqMap::new(), &core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_rps(
        &mut self,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatRps, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_rps(&mut CseqMap::new(), &core.u_data, item_uid, time_options, shield_perc)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_erps(
        &mut self,
        incoming_dps: Option<DpsProfile>,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatErps, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_erps(
                &mut CseqMap::new(),
                &core.u_data,
                item_uid,
                incoming_dps,
                time_options,
                shield_perc,
            )
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_breach_resist(&mut self) -> Result<UnitInterval, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_breach_resist(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - cap
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_cap_amount(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_cap_amount(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_cap_balance(
        &mut self,
        src_kinds: StatCapBlcSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<Value, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let src_kinds = StatCapBlcSrcKindsInt::from_pub(&src_kinds, &core.u_data)?;
        core.svc
            .get_stat_item_cap_balance(&mut CseqMap::new(), &core.u_data, item_uid, src_kinds, time_options)
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_cap_sim(
        &mut self,
        cap_perc: UnitInterval,
        optional_reloads: Option<OptionalReload>,
        stagger: &StatCapSimStagger,
        nosf_projectee_item_id: Option<&ItemId>,
    ) -> Result<StatCapSim, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let nosf_projectee_item_uid = match nosf_projectee_item_id {
            Some(nosf_projectee_item_id) => Some(core.u_data.get_projectee_uid(nosf_projectee_item_id)?),
            None => None,
        };
        core.svc
            .get_stat_item_cap_sim(
                &mut CseqMap::new(),
                &core.u_data,
                item_uid,
                cap_perc,
                optional_reloads,
                &StatCapSimStaggerInt::from_pub(core, stagger),
                nosf_projectee_item_uid,
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_neut_resist(&mut self) -> Result<UnitInterval, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_neut_resist(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - sensors
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_locks(&mut self) -> Result<Count, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_locks(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_lock_range(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_lock_range(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_scan_res(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_scan_res(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_sensors(&mut self) -> Result<StatSensors, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_sensors(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_dscan_range(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_dscan_range(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_probing_size(&mut self) -> Result<PValue, StatItemError<StatProbingSizeError>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_probing_size(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_incoming_jam(&mut self, time_options: StatTimeOptions) -> Result<StatInJam, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_incoming_jam(&mut CseqMap::new(), &core.u_data, item_uid, time_options)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - mobility
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_speed(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_speed(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_agility(&mut self) -> Result<PValue, StatItemError<StatAgilityError>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_agility(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_align_time(&mut self) -> Result<PValue, StatItemError<StatAgilityError>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_align_time(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_sig_radius(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_sig_radius(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_mass(&mut self, affectors: CtlAffectors) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let savepoint = core.internal_tx_savepoint();
        core.internal_ctl_affectors_switch(
            item_uid,
            core.u_data.r_data.get_attr_consts().mass,
            affectors,
            AffectionDir::Increase,
        );
        let result = core
            .svc
            .get_stat_item_mass(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items));
        core.internal_tx_rollback(savepoint);
        result
    }
    fn get_stat_warp_speed(&mut self) -> Result<PValue, StatItemError<StatWarpSpeedError>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_warp_speed(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_max_warp_range(&mut self) -> Result<PValue, StatItemError<StatMaxWarpRangeError>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_max_warp_range(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_jump(
        &mut self,
        range: StatJumpRange,
        passenger_fit_ids: &[FitId],
        passenger_fuel_affectors: CtlAffectors,
    ) -> Result<StatJump, StatItemError<StatJumpError>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let savepoint = core.internal_tx_savepoint();
        let passenger_fit_uids = passenger_fit_ids
            .iter()
            .filter_map(|fit_id| core.u_data.fits.int_id_by_ext_id(fit_id))
            .collect_vec();
        let passenger_ship_uids = core.u_data.get_fits_ship_uids(passenger_fit_uids.iter().copied());
        // Work on item states according to request
        for &passenger_ship_uid in passenger_ship_uids.iter() {
            core.internal_ctl_affectors_switch(
                passenger_ship_uid,
                core.u_data.r_data.get_attr_consts().mass,
                passenger_fuel_affectors,
                AffectionDir::Increase,
            );
        }
        // Collect stats
        let result = core
            .svc
            .get_stat_item_jump(&core.u_data, item_uid, range, &passenger_fit_uids)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items));
        // Revert item state changes
        core.internal_tx_rollback(savepoint);
        result
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - misc
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_drone_control_range(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_drone_control_range(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_can_warp(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_can_warp(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_can_jump_gate(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_can_jump_gate(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_can_jump_wormhole(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_can_jump_wormhole(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_can_jump_drive(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_can_jump_drive(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_can_dock_station(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_can_dock_station(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_can_dock_citadel(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_can_dock_citadel(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
    fn get_stat_can_tether(&mut self) -> Result<bool, StatItemError<!>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        core.svc
            .get_stat_item_can_tether(&core.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &core.u_data.items))
    }
}
