pub(in crate::api) use private::{ItemMutSealed, ItemSealed};

use super::err::{
    GetItemAttrError, ItemStatAppliedError, ItemStatError, IterItemAttrsError, IterItemEffectsError,
    IterItemModifiersError,
};
use crate::{
    api::{AttrId, AttrVals, EffectId, EffectInfo},
    def::{AttrVal, Count, ItemId, ItemTypeId},
    err::basic::{AttrFoundError, ItemLoadedError, ItemReceiveProjError},
    misc::{DmgKinds, DpsProfile, EffectMode, Spool},
    sol::SolarSystem,
    stats::StatCapSrcKinds,
    svc::{
        calc::Modification,
        vast::{
            StatCapSim, StatCapSimStagger, StatCapSimStaggerInt, StatDmg, StatDmgApplied, StatJamApplied, StatLayerEhp,
            StatLayerErps, StatLayerErpsRegen, StatLayerHp, StatLayerRps, StatLayerRpsRegen, StatMining, StatSensors,
            StatTank, StatTankRegen, StatTimeOptions,
        },
    },
    ud::{UEffectUpdates, UItemKey},
    util::UnitInterval,
};

mod private {
    use crate::{sol::SolarSystem, ud::UItemKey};

    pub trait ItemSealed: Sized {
        fn get_sol(&self) -> &SolarSystem;
        fn get_key(&self) -> UItemKey;
    }

    pub trait ItemMutSealed: ItemSealed {
        fn get_sol_mut(&mut self) -> &mut SolarSystem;
    }
}

pub trait ItemCommon: ItemSealed {
    fn get_item_id(&self) -> ItemId {
        self.get_sol().u_data.items.id_by_key(self.get_key())
    }
    fn get_type_id(&self) -> ItemTypeId {
        self.get_sol().u_data.items.get(self.get_key()).get_type_id()
    }
    fn iter_effects(&self) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        let sol = self.get_sol();
        let item_key = self.get_key();
        let item = sol.u_data.items.get(item_key);
        let (effect_keys, reffs) = match (item.get_effect_datas(), item.get_reffs()) {
            (Some(effect_datas), Some(reffs)) => (effect_datas.keys(), reffs),
            _ => {
                return Err(ItemLoadedError {
                    item_id: sol.u_data.items.id_by_key(item_key),
                }
                .into());
            }
        };
        let effect_infos = effect_keys.map(move |&effect_key| {
            let effect_id = sol.u_data.src.get_effect(effect_key).id;
            let running = reffs.contains(&effect_key);
            let mode = item.get_effect_key_mode(&effect_key);
            (effect_id.into(), EffectInfo { running, mode })
        });
        Ok(effect_infos)
    }
}

pub trait ItemMutCommon: ItemCommon + ItemMutSealed {
    fn get_attr(&mut self, attr_id: &AttrId) -> Result<AttrVals, GetItemAttrError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        let a_attr_id = attr_id.into();
        let attr_key = match sol.u_data.src.get_attr_key_by_id(&a_attr_id) {
            Some(attr_key) => attr_key,
            None => return Err(AttrFoundError { attr_id: *attr_id }.into()),
        };
        match sol.internal_get_item_attr(item_key, attr_key) {
            Ok(calc_val) => Ok(calc_val.into()),
            Err(error) => Err(ItemLoadedError {
                item_id: self.get_sol().u_data.items.id_by_key(error.item_key),
            }
            .into()),
        }
    }
    fn iter_attrs(&mut self) -> Result<impl ExactSizeIterator<Item = (AttrId, AttrVals)>, IterItemAttrsError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_attr_vals(&sol.u_data, item_key) {
            Ok(attr_iter) => {
                Ok(attr_iter.map(|(attr_key, val)| (sol.u_data.src.get_attr(attr_key).id.into(), val.into())))
            }
            Err(error) => Err(ItemLoadedError {
                item_id: sol.u_data.items.id_by_key(error.item_key),
            }
            .into()),
        }
    }
    fn iter_modifiers(
        &mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<Modification>)>, IterItemModifiersError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_mods(&sol.u_data, item_key) {
            Ok(mods_iter) => Ok(mods_iter),
            Err(err) => Err(ItemLoadedError {
                item_id: sol.u_data.items.id_by_key(err.item_key),
            }
            .into()),
        }
    }
    fn set_effect_mode(&mut self, effect_id: &EffectId, effect_mode: EffectMode) {
        let item_key = self.get_key();
        let mut reuse_eupdates = UEffectUpdates::new();
        self.get_sol_mut()
            .internal_set_effect_id_mode(item_key, effect_id.into(), effect_mode, &mut reuse_eupdates);
    }
    fn set_effect_modes(&mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) {
        let item_key = self.get_key();
        let mut reuse_eupdates = UEffectUpdates::new();
        self.get_sol_mut().internal_set_effect_id_modes(
            item_key,
            effect_modes.map(|(k, v)| (k.into(), v)),
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
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_dps_raw(&sol.u_data, item_key, reload, spool, include_charges, ignore_state)
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
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        let projectee_key = get_stat_applied_projectee_key(sol, projectee_item_id)?;
        sol.svc
            .get_stat_item_dps_applied(
                &sol.u_data,
                item_key,
                reload,
                spool,
                include_charges,
                ignore_state,
                projectee_key,
            )
            .map_err(|e| ItemStatAppliedError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_volley(
        &mut self,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_volley_raw(&sol.u_data, item_key, spool, include_charges, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_volley_applied(
        &mut self,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, ItemStatAppliedError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        let projectee_key = get_stat_applied_projectee_key(sol, projectee_item_id)?;
        sol.svc
            .get_stat_item_volley_applied(
                &sol.u_data,
                item_key,
                spool,
                include_charges,
                ignore_state,
                projectee_key,
            )
            .map_err(|e| ItemStatAppliedError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_mps(&mut self, reload: bool, ignore_state: bool) -> Result<StatMining, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_mps(&sol.u_data, item_key, reload, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_nps(&mut self, include_charges: bool, ignore_state: bool) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_outgoing_nps(&sol.u_data, item_key, include_charges, ignore_state, None)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_nps_applied(
        &mut self,
        include_charges: bool,
        ignore_state: bool,
        projectee_item_id: &ItemId,
    ) -> Result<AttrVal, ItemStatAppliedError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        let projectee_key = get_stat_applied_projectee_key(sol, projectee_item_id)?;
        sol.svc
            .get_stat_item_outgoing_nps(
                &sol.u_data,
                item_key,
                include_charges,
                ignore_state,
                Some(projectee_key),
            )
            .map_err(|e| ItemStatAppliedError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_rps(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_outgoing_rps(&sol.u_data, item_key, time_options, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_outgoing_cps(
        &mut self,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_outgoing_cps(&sol.u_data, item_key, time_options, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - tank
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_resists(&mut self) -> Result<StatTank<DmgKinds<AttrVal>>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_resists(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_hp(&mut self) -> Result<StatTank<StatLayerHp>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_hp(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_ehp(
        &mut self,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatTank<Option<StatLayerEhp>>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_ehp(&sol.u_data, item_key, incoming_dps)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_wc_ehp(&mut self) -> Result<StatTank<Option<StatLayerEhp>>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_wc_ehp(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_rps(
        &mut self,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatTankRegen<StatLayerRps, StatLayerRpsRegen>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_rps(&sol.u_data, item_key, time_options, shield_perc)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_erps(
        &mut self,
        incoming_dps: Option<DpsProfile>,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatTankRegen<Option<StatLayerErps>, Option<StatLayerErpsRegen>>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_erps(&sol.u_data, item_key, incoming_dps, time_options, shield_perc)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - cap
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_cap_amount(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_cap_amount(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_cap_balance(&mut self, src_kinds: StatCapSrcKinds) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_cap_balance(&sol.u_data, item_key, src_kinds)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_cap_sim(
        &mut self,
        cap_perc: UnitInterval,
        stagger: StatCapSimStagger,
    ) -> Result<StatCapSim, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_cap_sim(
                &sol.u_data,
                item_key,
                cap_perc,
                StatCapSimStaggerInt::from_pub(sol, &stagger),
            )
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_neut_resist(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_neut_resist(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - sensors
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_locks(&mut self) -> Result<Count, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_locks(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_lock_range(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_lock_range(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_scan_res(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_scan_res(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_sensors(&mut self) -> Result<StatSensors, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_sensors(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_dscan_range(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_dscan_range(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_probing_size(&mut self) -> Result<Option<AttrVal>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_probing_size(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_incoming_jam(&mut self) -> Result<StatJamApplied, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_incoming_jam(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - mobility
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_speed(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_speed(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_agility(&mut self) -> Result<Option<AttrVal>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_agility(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_align_time(&mut self) -> Result<Option<AttrVal>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_align_time(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_sig_radius(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_sig_radius(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_mass(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_mass(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_warp_speed(&mut self) -> Result<Option<AttrVal>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_warp_speed(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_max_warp_range(&mut self) -> Result<Option<AttrVal>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_max_warp_range(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Stats - misc
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_stat_drone_control_range(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_drone_control_range(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_warp(&mut self) -> Result<bool, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_warp(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_jump_gate(&mut self) -> Result<bool, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_jump_gate(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_jump_drive(&mut self) -> Result<bool, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_jump_drive(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_dock_station(&mut self) -> Result<bool, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_dock_station(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_dock_citadel(&mut self) -> Result<bool, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_dock_citadel(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_can_tether(&mut self) -> Result<bool, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_can_tether(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
}

fn get_stat_applied_projectee_key(
    sol: &SolarSystem,
    projectee_item_id: &ItemId,
) -> Result<UItemKey, ItemStatAppliedError> {
    let projectee_key = sol.u_data.items.key_by_id_err(projectee_item_id)?;
    let projectee_u_item = sol.u_data.items.get(projectee_key);
    if projectee_u_item.get_direct_physics().is_none() {
        return Err(ItemReceiveProjError {
            item_id: projectee_u_item.get_item_id(),
            item_kind: projectee_u_item.get_name(),
        }
        .into());
    }
    Ok(projectee_key)
}
