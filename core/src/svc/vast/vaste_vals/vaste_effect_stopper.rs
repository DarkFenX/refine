use std::collections::HashMap;

use crate::{
    def::{ItemId, ItemKey},
    misc::EffectId,
    sol::REffs,
    svc::{SvcCtx, vast::VastFitData},
    util::RSet,
};

pub struct ValEffectStopperFail {
    /// Items and their running effects which should be stopped.
    pub items: HashMap<ItemId, Vec<EffectId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_effect_stopper_fast(&self, kfs: &RSet<ItemKey>, reffs: &REffs) -> bool {
        for target_effect_spec in self.stopped_effects.keys() {
            if reffs.is_running(&target_effect_spec.item_key, &target_effect_spec.a_effect_id)
                && !kfs.contains(&target_effect_spec.item_key)
            {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_effect_stopper_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
        reffs: &REffs,
    ) -> Option<ValEffectStopperFail> {
        let mut items = HashMap::new();
        for target_effect_spec in self.stopped_effects.keys() {
            if reffs.is_running(&target_effect_spec.item_key, &target_effect_spec.a_effect_id)
                && !kfs.contains(&target_effect_spec.item_key)
            {
                items
                    .entry(ctx.uad.items.id_by_key(target_effect_spec.item_key))
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
