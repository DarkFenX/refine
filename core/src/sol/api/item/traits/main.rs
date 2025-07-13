pub(in crate::sol::api) use private::{ItemMutSealed, ItemSealed};

use super::err::{GetItemAttrError, ItemStatError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError};
use crate::{
    def::{AttrId, AttrVal, ItemId, ItemTypeId},
    err::basic::ItemLoadedError,
    misc::{DmgKinds, DpsProfile, EffectId, EffectInfo, EffectMode, Spool},
    sol::SolarSystem,
    svc::{
        calc::{CalcAttrVal, ModificationInfo},
        vast::{StatLayerEhp, StatLayerHp, StatLayerReps, StatTank},
    },
};

pub trait ItemCommon: ItemSealed {
    fn get_item_id(&self) -> ItemId {
        self.get_sol().uad.items.id_by_key(self.get_key())
    }
    fn get_type_id(&self) -> ItemTypeId {
        self.get_sol().uad.items.get(self.get_key()).get_a_item_id()
    }
    fn iter_effects(&self) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        let sol = self.get_sol();
        let item_key = self.get_key();
        let item = sol.uad.items.get(item_key);
        let a_effect_ids = match item.get_a_effect_datas() {
            Some(a_effect_datas) => a_effect_datas.keys(),
            None => {
                return Err(ItemLoadedError {
                    item_id: sol.uad.items.id_by_key(item_key),
                }
                .into());
            }
        };
        let effect_infos = a_effect_ids.map(move |a_effect_id| {
            let running = sol.reffs.is_running(&item_key, a_effect_id);
            let mode = *item.get_effect_modes().get(a_effect_id);
            (a_effect_id.into(), EffectInfo { running, mode })
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
                item_id: self.get_sol().uad.items.id_by_key(error.item_key),
            }
            .into()),
        }
    }
    fn iter_attrs(&mut self) -> Result<impl ExactSizeIterator<Item = (AttrId, CalcAttrVal)>, IterItemAttrsError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_attr_vals(&sol.uad, item_key) {
            Ok(attr_iter) => Ok(attr_iter),
            Err(error) => Err(ItemLoadedError {
                item_id: sol.uad.items.id_by_key(error.item_key),
            }
            .into()),
        }
    }
    fn iter_modifiers(
        &mut self,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<ModificationInfo>)>, IterItemModifiersError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_mods(&sol.uad, item_key) {
            Ok(mods_iter) => Ok(mods_iter),
            Err(err) => Err(ItemLoadedError {
                item_id: sol.uad.items.id_by_key(err.item_key),
            }
            .into()),
        }
    }
    fn set_effect_mode(&mut self, effect_id: &EffectId, mode: EffectMode)
    where
        Self: Sized,
    {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.uad
            .items
            .get_mut(item_key)
            .get_effect_modes_mut()
            .set(effect_id.into(), mode);
        let uad_item = sol.uad.items.get(item_key);
        SolarSystem::util_process_effects(
            &sol.uad,
            &mut sol.svc,
            &mut sol.reffs,
            item_key,
            uad_item,
            uad_item.get_a_state(),
        );
    }
    fn set_effect_modes(&mut self, modes: impl Iterator<Item = (EffectId, EffectMode)>)
    where
        Self: Sized,
    {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        let effect_modes = sol.uad.items.get_mut(item_key).get_effect_modes_mut();
        for (effect_id, effect_mode) in modes {
            effect_modes.set(effect_id.into(), effect_mode)
        }
        let uad_item = sol.uad.items.get(item_key);
        SolarSystem::util_process_effects(
            &sol.uad,
            &mut sol.svc,
            &mut sol.reffs,
            item_key,
            uad_item,
            uad_item.get_a_state(),
        );
    }
    // Stats - mobility
    fn get_stat_speed(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_speed(&sol.uad, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.uad.items, e))
    }
    fn get_stat_agility(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_agility(&sol.uad, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.uad.items, e))
    }
    fn get_stat_align_time(&mut self) -> Result<AttrVal, ItemStatError> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_align_time(&sol.uad, item_key)
            .map_err(|e| ItemStatError::from_svc_err(&sol.uad.items, e))
    }
    // Stats - tank
    fn get_stat_hp(&mut self) -> Option<StatTank<StatLayerHp>> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc.get_stat_item_hp(&sol.uad, item_key)
    }
    fn get_stat_ehp(&mut self, incoming_dps: Option<&DpsProfile>) -> Option<StatTank<StatLayerEhp>> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc.get_stat_item_ehp(&sol.uad, item_key, incoming_dps)
    }
    fn get_stat_wc_ehp(&mut self) -> Option<StatTank<StatLayerEhp>> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc.get_stat_item_wc_ehp(&sol.uad, item_key)
    }
    fn get_stat_reps(&mut self) -> Option<StatTank<StatLayerReps>> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc.get_stat_item_reps(&sol.uad, item_key)
    }
    fn get_stat_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc.get_stat_item_resists(&sol.uad, item_key)
    }
    // Stats - RR
    fn get_stat_rr_shield(&mut self, spool: Option<Spool>, ignore_state: bool) -> Option<AttrVal> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_orr_shield(&sol.uad, &sol.reffs, item_key, spool, ignore_state)
    }
    fn get_stat_rr_armor(&mut self, spool: Option<Spool>, ignore_state: bool) -> Option<AttrVal> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_orr_armor(&sol.uad, &sol.reffs, item_key, spool, ignore_state)
    }
    fn get_stat_rr_hull(&mut self, spool: Option<Spool>, ignore_state: bool) -> Option<AttrVal> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_orr_hull(&sol.uad, &sol.reffs, item_key, spool, ignore_state)
    }
    fn get_stat_rr_capacitor(&mut self, spool: Option<Spool>, ignore_state: bool) -> Option<AttrVal> {
        let item_key = self.get_key();
        let sol = self.get_sol_mut();
        sol.svc
            .get_stat_item_orr_cap(&sol.uad, &sol.reffs, item_key, spool, ignore_state)
    }
}

mod private {
    use crate::{def::ItemKey, sol::SolarSystem};

    pub trait ItemSealed: Sized {
        fn get_sol(&self) -> &SolarSystem;
        fn get_key(&self) -> ItemKey;
    }

    pub trait ItemMutSealed: ItemSealed {
        fn get_sol_mut(&mut self) -> &mut SolarSystem;
    }
}
