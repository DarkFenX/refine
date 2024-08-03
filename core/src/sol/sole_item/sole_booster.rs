use itertools::Itertools;

use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolBooster, SolItem},
        item_info::{SolBoosterInfo, SolSideEffectInfo, SolSideEffectStr},
        SolarSystem,
    },
    util::{Result, StMap},
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
        self.items.get_booster_mut(item_id)?.set_bool_state(state);
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
                        let status = self.svcs.is_effect_running(&booster.base.id, effect_id);
                        let side_effect = SolSideEffectInfo::new(chance_attr_id, status, se_str);
                        side_effects.insert(*effect_id, side_effect);
                    }
                }
            }
        }
        SolBoosterInfo::from_booster_and_side_effects(booster, side_effects)
    }
}
