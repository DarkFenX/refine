use std::collections::HashMap;

use crate::{
    sol::{
        EffectId, ItemId, ItemKey,
        svc::{running_effects::RunningEffects, vast::VastFitData},
        uad::Uad,
    },
    util::RSet,
};

pub struct ValEffectStopperFail {
    /// Items and their running effects which should be stopped.
    pub items: HashMap<ItemId, Vec<EffectId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_effect_stopper_fast(
        &self,
        kfs: &RSet<ItemKey>,
        running_effects: &RunningEffects,
    ) -> bool {
        for target_effect_spec in self.stopped_effects.keys() {
            if running_effects.is_running(&target_effect_spec.item_key, &target_effect_spec.a_effect_id)
                && !kfs.contains(&target_effect_spec.item_key)
            {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_effect_stopper_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        running_effects: &RunningEffects,
    ) -> Option<ValEffectStopperFail> {
        let mut items = HashMap::new();
        for target_effect_spec in self.stopped_effects.keys() {
            if running_effects.is_running(&target_effect_spec.item_key, &target_effect_spec.a_effect_id)
                && !kfs.contains(&target_effect_spec.item_key)
            {
                items
                    .entry(uad.items.id_by_key(target_effect_spec.item_key))
                    .or_insert_with(Vec::new)
                    .push(target_effect_spec.a_effect_id.into());
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValEffectStopperFail { items }),
        }
    }
}
