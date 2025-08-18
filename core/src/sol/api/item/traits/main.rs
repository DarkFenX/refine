pub(in crate::sol::api) use private::{ItemMutSealed, ItemSealed};

use super::err::{GetItemAttrError, ItemStatError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError};
use crate::{
    def::{AttrId, AttrVal, ItemId, ItemTypeId},
    err::basic::ItemLoadedError,
    misc::{DmgKinds, DpsProfile, EffectId, EffectInfo, EffectMode, Spool},
    svc::{
        calc::{CalcAttrVal, ModificationInfo},
        vast::{StatDmg, StatLayerEhp, StatLayerErps, StatLayerHp, StatLayerRps, StatTank},
    },
    ud::UEffectUpdates,
    util::GetId,
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
            let effect_id = sol.u_data.src.get_effect(effect_key).get_id();
            let running = reffs.contains(&effect_key);
            let mode = item.get_effect_key_mode(&effect_key);
            (effect_id.into(), EffectInfo { running, mode })
        });
        Ok(effect_infos)
    }
}

pub trait ItemMutCommon: ItemCommon + ItemMutSealed {
    fn get_attr(&mut self, attr_id: &AttrId) -> Result<CalcAttrVal, GetItemAttrError> {
        let item_key = self.get_key();
        match self.get_sol_mut().internal_get_item_attr(item_key, attr_id) {
            Ok(calc_val) => Ok(calc_val),
            Err(error) => Err(ItemLoadedError {
                item_id: self.get_sol().u_data.items.id_by_key(error.item_key),
            }
            .into()),
        }
    }
    fn iter_attrs(&mut self) -> Result<impl ExactSizeIterator<Item = (AttrId, CalcAttrVal)>, IterItemAttrsError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_attr_vals(&sol.u_data, item_key) {
            Ok(attr_iter) => Ok(attr_iter),
            Err(error) => Err(ItemLoadedError {
                item_id: sol.u_data.items.id_by_key(error.item_key),
            }
            .into()),
        }
    }
    fn iter_modifiers(
        &mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
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
    fn set_effect_mode(&mut self, effect_id: &EffectId, effect_mode: EffectMode)
    where
        Self: Sized,
    {
        let item_key = self.get_key();
        let mut reuse_eupdates = UEffectUpdates::new();
        self.get_sol_mut()
            .internal_set_effect_id_mode(item_key, effect_id.into(), effect_mode, &mut reuse_eupdates);
    }
    fn set_effect_modes(&mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>)
    where
        Self: Sized,
    {
        let item_key = self.get_key();
        let mut reuse_eupdates = UEffectUpdates::new();
        self.get_sol_mut().internal_set_effect_id_modes(
            item_key,
            effect_modes.map(|(k, v)| (k.into(), v)),
            &mut reuse_eupdates,
        );
    }
    // Stats - mobility
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
    // Stats - damage
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
            .get_stat_item_dps(&sol.u_data, item_key, reload, spool, include_charges, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
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
            .get_stat_item_volley(&sol.u_data, item_key, spool, include_charges, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    // Stats - tank
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
    fn get_stat_rps(&mut self, spool: Option<Spool>) -> Result<StatTank<StatLayerRps>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_rps(&sol.u_data, item_key, spool)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_erps(
        &mut self,
        incoming_dps: Option<DpsProfile>,
        spool: Option<Spool>,
    ) -> Result<StatTank<Option<StatLayerErps>>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_erps(&sol.u_data, item_key, incoming_dps, spool)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_resists(&mut self) -> Result<StatTank<DmgKinds<AttrVal>>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_resists(&sol.u_data, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    // Stats - RR
    fn get_stat_remote_rps(
        &mut self,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_remote_rps(&sol.u_data, item_key, spool, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
    fn get_stat_remote_cps(&mut self, ignore_state: bool) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_remote_cps(&sol.u_data, item_key, ignore_state)
            .map_err(|e| ItemStatError::from_svc_err(&sol.u_data.items, e))
    }
}
