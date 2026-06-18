use super::{
    err::{
        GetItemAttrError, ItemAppliedStatError, ItemStatError, IterItemAttrsError, IterItemEffectsError,
        IterItemModifiersError,
    },
    sealed::{ItemMutSealed, ItemSealed},
};
use crate::{
    api::{AttrId, AttrVals, EffectId, EffectInfo, ItemTypeId},
    err::basic::{AttrFoundError, ItemLoadedError},
    misc::{DpsProfile, EffectMode, OptionalReload},
    num::{Count, PValue, UnitInterval, Value},
    svc::{
        calc::Modification,
        cycle::CseqMap,
        vast::{
            StatCapBlcSrcKinds, StatCapBlcSrcKindsInt, StatCapSim, StatCapSimStagger, StatCapSimStaggerInt, StatDmg,
            StatDmgApplied, StatEhp, StatErps, StatHp, StatInJam, StatMining, StatOutReps, StatResists, StatRps,
            StatSensors, StatTimeOptions,
        },
    },
    ud::{ItemId, UEffectUpdates},
};

#[allow(private_bounds)]
pub trait ItemCommon: ItemSealed {
    fn get_item_id(&self) -> ItemId {
        self.get_sol().u_data.items.ext_id_by_int_id(self.get_uid())
    }
    fn get_type_id(&self) -> ItemTypeId {
        let type_aid = self.get_sol().u_data.items.get(self.get_uid()).get_type_aid();
        ItemTypeId::from_aid(type_aid)
    }
    fn iter_effects(&self) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        let sol = self.get_sol();
        let item_uid = self.get_uid();
        let item = sol.u_data.items.get(item_uid);
        let (effect_rids, reffs) = match (item.get_effects(), item.get_reffs()) {
            (Some(effects), Some(reffs)) => (effects.keys(), reffs),
            _ => {
                return Err(ItemLoadedError {
                    item_id: sol.u_data.items.ext_id_by_int_id(item_uid),
                }
                .into());
            }
        };
        let effect_infos = effect_rids.map(move |&effect_rid| {
            let effect_aid = sol.u_data.src.get_effect_by_rid(effect_rid).aid;
            let running = reffs.contains(&effect_rid);
            let mode = item.get_effect_mode(&effect_rid);
            (EffectId::from_aid(effect_aid), EffectInfo { running, mode })
        });
        Ok(effect_infos)
    }
}

#[allow(private_bounds)]
pub trait ItemMutCommon: ItemCommon + ItemMutSealed {
    fn get_attr(&mut self, attr_id: &AttrId) -> Result<AttrVals, GetItemAttrError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let attr_aid = attr_id.into_aid();
        let Some(attr_rid) = sol.u_data.src.get_attr_rid_by_aid(&attr_aid) else {
            return Err(AttrFoundError { attr_id: *attr_id }.into());
        };
        match sol.internal_get_item_attr(item_uid, attr_rid) {
            Ok(calc_vals) => Ok(AttrVals::from_calc_attr_vals(calc_vals)),
            Err(error) => Err(ItemLoadedError {
                item_id: self.get_sol().u_data.items.ext_id_by_int_id(error.item_uid),
            }
            .into()),
        }
    }
    fn iter_attrs(&mut self) -> Result<impl ExactSizeIterator<Item = (AttrId, AttrVals)>, IterItemAttrsError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_attr_vals(&sol.u_data, item_uid) {
            Ok(attr_iter) => Ok(attr_iter.map(|(attr_rid, calc_vals)| {
                (
                    AttrId::from_aid(sol.u_data.src.get_attr_by_rid(attr_rid).aid),
                    AttrVals::from_calc_attr_vals(calc_vals),
                )
            })),
            Err(error) => Err(ItemLoadedError {
                item_id: sol.u_data.items.ext_id_by_int_id(error.item_uid),
            }
            .into()),
        }
    }
    fn iter_modifiers(
        &mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<Modification>)>, IterItemModifiersError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_mods(&sol.u_data, item_uid) {
            Ok(mods_iter) => Ok(mods_iter),
            Err(err) => Err(ItemLoadedError {
                item_id: sol.u_data.items.ext_id_by_int_id(err.item_uid),
            }
            .into()),
        }
    }
    fn set_effect_mode(&mut self, effect_id: &EffectId, effect_mode: EffectMode) {
        let item_uid = self.get_uid();
        let mut reuse_eupdates = UEffectUpdates::new();
        self.get_sol_mut().internal_set_effect_id_mode(
            item_uid,
            effect_id.into_aid(),
            effect_mode,
            &mut reuse_eupdates,
        );
    }
    fn set_effect_modes(&mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) {
        let item_uid = self.get_uid();
        let mut reuse_eupdates = UEffectUpdates::new();
        self.get_sol_mut().internal_set_effect_id_modes(
            item_uid,
            effect_modes.map(|(effect_id, effect_mode)| (effect_id.into_aid(), effect_mode)),
            &mut reuse_eupdates,
        );
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - output
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_dmg(
        &mut self,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, ItemStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(include_charges, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let result = sol
            .svc
            .get_stat_item_dmg_raw(
                &mut CseqMap::new(),
                &sol.u_data,
                item_uid,
                time_options,
                include_charges,
            )
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    fn get_stat_dmg_applied(
        &mut self,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, ItemAppliedStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(include_charges, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = sol.u_data.get_projectee_uid(projectee_item_id)?;
        let result = sol
            .svc
            .get_stat_item_dmg_applied(
                &mut CseqMap::new(),
                &sol.u_data,
                item_uid,
                time_options,
                include_charges,
                projectee_uid,
            )
            .map_err(|e| ItemAppliedStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    fn get_stat_mps(
        &mut self,
        time_options: StatTimeOptions,
        mission_ore: bool,
        ignore_state: bool,
    ) -> Result<StatMining, ItemStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(false, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let result = sol
            .svc
            .get_stat_item_mps(&mut CseqMap::new(), &sol.u_data, item_uid, time_options, mission_ore)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    fn get_stat_outgoing_nps(
        &mut self,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<PValue, ItemStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(include_charges, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let result = sol
            .svc
            .get_stat_item_outgoing_nps(
                &mut CseqMap::new(),
                &sol.u_data,
                item_uid,
                time_options,
                include_charges,
                None,
            )
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    fn get_stat_outgoing_nps_applied(
        &mut self,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, ItemAppliedStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(include_charges, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = sol.u_data.get_projectee_uid(projectee_item_id)?;
        let result = sol
            .svc
            .get_stat_item_outgoing_nps(
                &mut CseqMap::new(),
                &sol.u_data,
                item_uid,
                time_options,
                include_charges,
                Some(projectee_uid),
            )
            .map_err(|e| ItemAppliedStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    fn get_stat_outgoing_rps(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<StatOutReps, ItemStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(false, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let result = sol
            .svc
            .get_stat_item_outgoing_rps(&mut CseqMap::new(), &sol.u_data, item_uid, time_options, None)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    fn get_stat_outgoing_rps_applied(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<StatOutReps, ItemAppliedStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(false, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = sol.u_data.get_projectee_uid(projectee_item_id)?;
        let result = sol
            .svc
            .get_stat_item_outgoing_rps(
                &mut CseqMap::new(),
                &sol.u_data,
                item_uid,
                time_options,
                Some(projectee_uid),
            )
            .map_err(|e| ItemAppliedStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    fn get_stat_outgoing_cps(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<PValue, ItemStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(false, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let result = sol
            .svc
            .get_stat_item_outgoing_cps(&mut CseqMap::new(), &sol.u_data, item_uid, time_options, None)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, ItemAppliedStatError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = self.active_stat_prepare(false, ignore_state, &mut reuse_eupdates);
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = sol.u_data.get_projectee_uid(projectee_item_id)?;
        let result = sol
            .svc
            .get_stat_item_outgoing_cps(
                &mut CseqMap::new(),
                &sol.u_data,
                item_uid,
                time_options,
                Some(projectee_uid),
            )
            .map_err(|e| ItemAppliedStatError::from_svc_err(e, &sol.u_data.items));
        self.active_stat_rollback(saved_state, &mut reuse_eupdates);
        result
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - tank
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_resists(&mut self) -> Result<StatResists, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_resists(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_hp(&mut self) -> Result<StatHp, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_hp(&mut CseqMap::new(), &sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_ehp(&mut self, incoming_dps: Option<DpsProfile>) -> Result<StatEhp, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_ehp(&mut CseqMap::new(), &sol.u_data, item_uid, incoming_dps)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_wc_ehp(&mut self) -> Result<StatEhp, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_wc_ehp(&mut CseqMap::new(), &sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_rps(
        &mut self,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatRps, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_rps(&mut CseqMap::new(), &sol.u_data, item_uid, time_options, shield_perc)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_erps(
        &mut self,
        incoming_dps: Option<DpsProfile>,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatErps, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_erps(
                &mut CseqMap::new(),
                &sol.u_data,
                item_uid,
                incoming_dps,
                time_options,
                shield_perc,
            )
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - cap
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_cap_amount(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_cap_amount(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_cap_balance(
        &mut self,
        src_kinds: &StatCapBlcSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<Value, ItemAppliedStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let src_kinds = StatCapBlcSrcKindsInt::from_pub(src_kinds, &sol.u_data)?;
        sol.svc
            .get_stat_item_cap_balance(&mut CseqMap::new(), &sol.u_data, item_uid, src_kinds, time_options)
            .map_err(|e| ItemAppliedStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_cap_sim(
        &mut self,
        cap_perc: UnitInterval,
        optional_reloads: Option<OptionalReload>,
        stagger: StatCapSimStagger,
        nosf_projectee_item_id: Option<&ItemId>,
    ) -> Result<StatCapSim, ItemAppliedStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let nosf_projectee_item_uid = match nosf_projectee_item_id {
            Some(nosf_projectee_item_id) => Some(sol.u_data.get_projectee_uid(nosf_projectee_item_id)?),
            None => None,
        };
        sol.svc
            .get_stat_item_cap_sim(
                &mut CseqMap::new(),
                &sol.u_data,
                item_uid,
                cap_perc,
                optional_reloads,
                &StatCapSimStaggerInt::from_pub(sol, &stagger),
                nosf_projectee_item_uid,
            )
            .map_err(|e| ItemAppliedStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_neut_resist(&mut self) -> Result<UnitInterval, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_neut_resist(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - sensors
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_locks(&mut self) -> Result<Count, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_locks(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_lock_range(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_lock_range(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_scan_res(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_scan_res(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_sensors(&mut self) -> Result<StatSensors, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_sensors(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_dscan_range(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_dscan_range(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_probing_size(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_probing_size(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_incoming_jam(&mut self, time_options: StatTimeOptions) -> Result<StatInJam, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_incoming_jam(&mut CseqMap::new(), &sol.u_data, item_uid, time_options)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - mobility
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_speed(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_speed(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_agility(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_agility(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_align_time(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_align_time(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_sig_radius(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_sig_radius(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_mass(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_mass(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_warp_speed(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_warp_speed(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_max_warp_range(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_max_warp_range(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - misc
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_drone_control_range(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_drone_control_range(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_can_warp(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_warp(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_can_jump_gate(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_jump_gate(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_can_jump_wormhole(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_jump_wormhole(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_can_jump_drive(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_jump_drive(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_can_dock_station(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_dock_station(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_can_dock_citadel(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_dock_citadel(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_can_tether(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_tether(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(e, &sol.u_data.items))
    }
}
