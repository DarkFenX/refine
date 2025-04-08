use std::collections::HashMap;

use itertools::Itertools;

use crate::sol::{
    EffectId, EffectMode, FitId, ItemId, ItemTypeId, SlotIndex,
    info::{SideEffectInfo, SideEffectStr},
    uad::{Uad, item::Booster},
};

pub struct BoosterInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
    pub side_effects: HashMap<EffectId, SideEffectInfo>,
}
impl BoosterInfo {
    pub(in crate::sol) fn from_booster(uad: &Uad, booster: &Booster) -> Self {
        let mut side_effects = HashMap::new();
        if let Some(a_effect_datas) = booster.get_a_effect_datas() {
            for a_effect_id in a_effect_datas.keys() {
                if let Some(a_effect) = uad.src.get_a_effect(a_effect_id) {
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
        Self {
            id: booster.get_item_id(),
            type_id: booster.get_a_item_id(),
            fit_id: uad.fits.id_by_key(booster.get_fit_key()),
            slot: booster.get_a_slot(),
            enabled: booster.get_booster_state(),
            side_effects,
        }
    }
}
