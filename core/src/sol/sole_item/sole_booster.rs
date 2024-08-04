use itertools::Itertools;

use crate::{
    defs::{EEffectId, EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolBooster, SolItem},
        item_info::{SolBoosterInfo, SolSideEffectInfo, SolSideEffectStr},
        view::SolView,
        SolEffectMode, SolarSystem,
    },
    util::{Error, ErrorKind, Result, StMap},
};

impl SolarSystem {
    // Public
    pub fn get_booster_info(&self, item_id: &SolItemId) -> Result<SolBoosterInfo> {
        Ok(self.make_booster_info(self.items.get_booster(item_id)?))
    }
    pub fn get_fit_booster_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolBoosterInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let booster_infos = fit
            .boosters
            .iter()
            .map(|v| self.make_booster_info(self.items.get_booster(v).unwrap()))
            .collect();
        Ok(booster_infos)
    }
    pub fn add_booster(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolBoosterInfo> {
        let item_id = self.items.alloc_item_id()?;
        let booster = SolBooster::new(&self.src, item_id, fit_id, a_item_id, state);
        let item = SolItem::Booster(booster);
        self.add_item(item);
        let info = self.get_booster_info(&item_id).unwrap();
        Ok(info)
    }
    pub fn set_booster_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        let booster = self.items.get_booster_mut(item_id)?;
        let old_state = booster.state;
        booster.set_bool_state(state);
        let new_state = booster.state;
        if new_state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                item,
                old_state,
                new_state,
            );
        }
        Ok(())
    }
    pub fn set_booster_side_effect_state(
        &mut self,
        item_id: &SolItemId,
        effect_id: &EEffectId,
        state: bool,
    ) -> Result<()> {
        let booster = self.items.get_booster_mut(item_id)?;
        let a_item = booster.get_a_item()?;
        if !a_item.effect_datas.contains_key(effect_id) {
            return Err(Error::new(ErrorKind::NotSideEffect(*effect_id)));
        }
        let effect = match self.src.get_a_effect(effect_id) {
            Some(effect) => effect,
            None => return Err(Error::new(ErrorKind::NotSideEffect(*effect_id))),
        };
        if effect.chance_attr_id.is_none() {
            return Err(Error::new(ErrorKind::NotSideEffect(*effect_id)));
        }
        let effect_state = match state {
            true => SolEffectMode::StateCompliance,
            false => SolEffectMode::FullCompliance,
        };
        booster.get_effect_modes_mut().set(*effect_id, effect_state);
        let item = self.items.get_item(item_id).unwrap();
        self.svcs.process_effects(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item,
            item.get_state(),
        );
        Ok(())
    }

    // Non-public
    pub(in crate::sol) fn make_booster_info(&self, booster: &SolBooster) -> SolBoosterInfo {
        let mut side_effects = StMap::new();
        if let Ok(a_item) = booster.get_a_item() {
            for effect_id in a_item.effect_datas.keys() {
                if let Some(effect) = self.src.get_a_effect(effect_id) {
                    if let Some(chance_attr_id) = effect.chance_attr_id {
                        let se_strs = effect
                            .mods
                            .iter()
                            .map(|v| SolSideEffectStr::new(v.op, v.affector_attr_id))
                            .collect_vec();
                        // Expose strength info only if all modifiers use the same source attribute
                        // and operator
                        let se_str = match se_strs.len() {
                            0 => None,
                            1 => se_strs.into_iter().next(),
                            _ => {
                                let first = *se_strs.first().unwrap();
                                match se_strs.iter().all(|se_str| *se_str == first) {
                                    true => Some(first),
                                    false => None,
                                }
                            }
                        };
                        let status = match booster.get_effect_modes().get(effect_id) {
                            SolEffectMode::FullCompliance => false,
                            SolEffectMode::StateCompliance => true,
                            SolEffectMode::ForceRun => true,
                            SolEffectMode::ForceStop => false,
                        };
                        let side_effect = SolSideEffectInfo::new(chance_attr_id, status, se_str);
                        side_effects.insert(*effect_id, side_effect);
                    }
                }
            }
        }
        SolBoosterInfo::from_booster_and_side_effects(booster, side_effects)
    }
}
