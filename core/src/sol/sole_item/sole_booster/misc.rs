use itertools::Itertools;

use crate::{
    sol::{
        EffectMode, SolarSystem,
        info::{BoosterInfo, SideEffectInfo, SideEffectStr},
        uad::item::Booster,
    },
    util::RMap,
};

impl SolarSystem {
    pub(in crate::sol) fn make_booster_info(&self, booster: &Booster) -> BoosterInfo {
        let mut side_effects = std::collections::HashMap::new();
        if let Some(a_effect_datas) = booster.get_a_effect_datas() {
            for a_effect_id in a_effect_datas.keys() {
                if let Some(a_effect) = self.uad.src.get_a_effect(a_effect_id) {
                    if let Some(chance_a_attr_id) = a_effect.chance_attr_id {
                        let se_strs = a_effect
                            .mods
                            .iter()
                            .map(|v| SideEffectStr {
                                op: v.op.into(),
                                attr_id: v.affector_attr_id,
                            })
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
                        let status = match booster.get_effect_modes().get(a_effect_id) {
                            EffectMode::FullCompliance => false,
                            EffectMode::StateCompliance => true,
                            EffectMode::ForceRun => true,
                            EffectMode::ForceStop => false,
                        };
                        let side_effect = SideEffectInfo {
                            chance_attr_id: chance_a_attr_id,
                            status,
                            strength: se_str,
                        };
                        side_effects.insert(a_effect_id.into(), side_effect);
                    }
                }
            }
        }
        BoosterInfo::from_booster_and_side_effects(booster, side_effects)
    }
}
