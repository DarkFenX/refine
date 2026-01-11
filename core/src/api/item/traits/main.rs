pub(in crate::api) use private::{ItemMutSealed, ItemSealed};

use super::err::{
    GetItemAttrError, ItemStatAppliedError, ItemStatError, IterItemAttrsError, IterItemEffectsError,
    IterItemModifiersError,
};
use crate::{
    api::{AttrId, AttrVals, EffectId, EffectInfo, ItemTypeId},
    err::basic::{AttrFoundError, ItemLoadedError, ItemReceiveProjError},
    misc::{DpsProfile, EffectMode, Spool},
    num::{Count, PValue, UnitInterval, Value},
    sol::SolarSystem,
    stats::StatCapSrcKinds,
    svc::{
        calc::Modification,
        vast::{
            StatCapSim, StatCapSimStagger, StatCapSimStaggerInt, StatDmg, StatDmgApplied, StatEhp, StatErps, StatHp,
            StatInJam, StatMining, StatOutReps, StatResists, StatRps, StatSensors, StatTimeOptions,
        },
    },
    ud::{ItemId, UEffectUpdates, UItemId},
};

mod private {
    use crate::{sol::SolarSystem, ud::UItemId};

    pub(crate) trait ItemSealed: Sized {
        fn get_sol(&self) -> &SolarSystem;
        fn get_uid(&self) -> UItemId;
    }

    pub(crate) trait ItemMutSealed: ItemSealed {
        fn get_sol_mut(&mut self) -> &mut SolarSystem;
    }
}

#[allow(private_bounds)]
pub trait ItemCommon: ItemSealed {
    fn get_item_id(&self) -> ItemId {
        self.get_sol().u_data.items.xid_by_iid(self.get_uid())
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
                    item_id: sol.u_data.items.xid_by_iid(item_uid),
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
        let attr_rid = match sol.u_data.src.get_attr_rid_by_aid(&attr_aid) {
            Some(attr_rid) => attr_rid,
            None => return Err(AttrFoundError { attr_id: *attr_id }.into()),
        };
        match sol.internal_get_item_attr(item_uid, attr_rid) {
            Ok(calc_vals) => Ok(AttrVals::from_calc_attr_vals(calc_vals)),
            Err(error) => Err(ItemLoadedError {
                item_id: self.get_sol().u_data.items.xid_by_iid(error.item_uid),
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
                item_id: sol.u_data.items.xid_by_iid(error.item_uid),
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
                item_id: sol.u_data.items.xid_by_iid(err.item_uid),
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
    fn get_stat_dps(
        &mut self,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_dps_raw(&sol.u_data, item_uid, reload, spool, include_charges, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_dps_applied(
        &mut self,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, ItemStatAppliedError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = get_stat_applied_projectee_uid(sol, projectee_item_id)?;
        sol.svc
            .get_stat_item_dps_applied(
                &sol.u_data,
                item_uid,
                reload,
                spool,
                include_charges,
                ignore_state,
                projectee_uid,
            )
            .map_err(|e| ItemStatAppliedError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_volley(
        &mut self,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_volley_raw(&sol.u_data, item_uid, spool, include_charges, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_volley_applied(
        &mut self,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, ItemStatAppliedError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = get_stat_applied_projectee_uid(sol, projectee_item_id)?;
        sol.svc
            .get_stat_item_volley_applied(
                &sol.u_data,
                item_uid,
                spool,
                include_charges,
                ignore_state,
                projectee_uid,
            )
            .map_err(|e| ItemStatAppliedError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_mps(&mut self, time_options: StatTimeOptions, ignore_state: bool) -> Result<StatMining, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_mps(&sol.u_data, item_uid, time_options, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_nps(
        &mut self,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_outgoing_nps(&sol.u_data, item_uid, time_options, include_charges, ignore_state, None)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_nps_applied(
        &mut self,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, ItemStatAppliedError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = get_stat_applied_projectee_uid(sol, projectee_item_id)?;
        sol.svc
            .get_stat_item_outgoing_nps(
                &sol.u_data,
                item_uid,
                time_options,
                include_charges,
                ignore_state,
                Some(projectee_uid),
            )
            .map_err(|e| ItemStatAppliedError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_rps(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<StatOutReps, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_outgoing_rps(&sol.u_data, item_uid, time_options, ignore_state, None)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_rps_applied(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<StatOutReps, ItemStatAppliedError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = get_stat_applied_projectee_uid(sol, projectee_item_id)?;
        sol.svc
            .get_stat_item_outgoing_rps(&sol.u_data, item_uid, time_options, ignore_state, Some(projectee_uid))
            .map_err(|e| ItemStatAppliedError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_cps(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_outgoing_cps(&sol.u_data, item_uid, time_options, ignore_state, None)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, ItemStatAppliedError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let projectee_uid = get_stat_applied_projectee_uid(sol, projectee_item_id)?;
        sol.svc
            .get_stat_item_outgoing_cps(&sol.u_data, item_uid, time_options, ignore_state, Some(projectee_uid))
            .map_err(|e| ItemStatAppliedError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - tank
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_resists(&mut self) -> Result<StatResists, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_resists(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_hp(&mut self) -> Result<StatHp, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_hp(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_ehp(&mut self, incoming_dps: Option<DpsProfile>) -> Result<StatEhp, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_ehp(&sol.u_data, item_uid, incoming_dps)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_wc_ehp(&mut self) -> Result<StatEhp, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_wc_ehp(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_rps(
        &mut self,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatRps, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_rps(&sol.u_data, item_uid, time_options, shield_perc)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
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
            .get_stat_item_erps(&sol.u_data, item_uid, incoming_dps, time_options, shield_perc)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - cap
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_cap_amount(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_cap_amount(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_cap_balance(
        &mut self,
        src_kinds: StatCapSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<Value, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_cap_balance(&sol.u_data, item_uid, src_kinds, time_options)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_cap_sim(
        &mut self,
        cap_perc: UnitInterval,
        reload_optionals: Option<bool>,
        stagger: StatCapSimStagger,
    ) -> Result<StatCapSim, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_cap_sim(
                &sol.u_data,
                item_uid,
                cap_perc,
                reload_optionals,
                StatCapSimStaggerInt::from_pub(sol, &stagger),
            )
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_neut_resist(&mut self) -> Result<UnitInterval, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_neut_resist(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - sensors
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_locks(&mut self) -> Result<Count, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_locks(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_lock_range(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_lock_range(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_scan_res(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_scan_res(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_sensors(&mut self) -> Result<StatSensors, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_sensors(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_dscan_range(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_dscan_range(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_probing_size(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_probing_size(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_incoming_jam(&mut self) -> Result<StatInJam, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_incoming_jam(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - mobility
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_speed(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_speed(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_agility(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_agility(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_align_time(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_align_time(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_sig_radius(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_sig_radius(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_mass(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_mass(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_warp_speed(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_warp_speed(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_max_warp_range(&mut self) -> Result<Option<PValue>, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_max_warp_range(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - misc
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_drone_control_range(&mut self) -> Result<PValue, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_drone_control_range(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_warp(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_warp(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_jump_gate(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_jump_gate(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_jump_drive(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_jump_drive(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_dock_station(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_dock_station(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_dock_citadel(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_dock_citadel(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_tether(&mut self) -> Result<bool, ItemStatError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_tether(&sol.u_data, item_uid)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
}

fn get_stat_applied_projectee_uid(
    sol: &SolarSystem,
    projectee_item_id: &ItemId,
) -> Result<UItemId, ItemStatAppliedError> {
    let projectee_uid = sol.u_data.items.iid_by_xid_err(projectee_item_id)?;
    let projectee_u_item = sol.u_data.items.get(projectee_uid);
    if projectee_u_item.get_direct_physics().is_none() {
        return Err(ItemReceiveProjError {
            item_id: projectee_u_item.get_item_id(),
            item_kind: projectee_u_item.lib_get_name(),
        }
        .into());
    }
    Ok(projectee_uid)
}
