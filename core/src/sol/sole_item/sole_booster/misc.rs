use itertools::Itertools;

use crate::{
    sol::{
        info::{SolBoosterInfo, SolSideEffectInfo, SolSideEffectStr},
        uad::item::SolBooster,
        SolEffectMode, SolarSystem,
    },
    util::StMap,
};

impl SolarSystem {
    pub(in crate::sol) fn make_booster_info(&self, booster: &SolBooster) -> SolBoosterInfo {
        let mut side_effects = StMap::new();
        if let Some(effect_datas) = booster.get_effect_datas() {
            for effect_id in effect_datas.keys() {
                if let Some(effect) = self.uad.src.get_a_effect(effect_id) {
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
