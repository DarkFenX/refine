use std::sync::Arc;

use crate::{
    ad::AItemId,
    rd::{
        REffectConsts, REffectId, RItemEffectOptions, RItemEffectsPregenEntry, RItemEffectsPregenId, RcEffect, RcItem,
    },
    util::{DenseId, PSlab, RMap},
};

/// Effects every item type runs without effect mode overrides, per effect resolution options.
pub(crate) struct RItemsEffectsPregen {
    // Indexed by IDs item types store
    data: Vec<RItemEffectsPregenEntry>,
}
impl RItemsEffectsPregen {
    pub(crate) fn get(&self, effects_pregen_id: RItemEffectsPregenId) -> &RItemEffectsPregenEntry {
        &self.data[effects_pregen_id.index()]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemsEffectsPregen {
    /// Resolves effects of every item type for every effect resolution options, and sets IDs of the
    /// results on item types.
    pub(in crate::rd::data) fn new(
        items: &mut RMap<AItemId, RcItem>,
        effects: &PSlab<REffectId, RcEffect>,
        effect_consts: &REffectConsts,
    ) -> Self {
        // Equal results are stored once
        let mut value_ids = RMap::new();
        // Allocating it per item type and options leaves the heap fragmented after load
        let mut reuse_state_effects = Vec::new();
        for r_item in items.values_mut() {
            let r_item = Arc::get_mut(r_item).unwrap();
            for effect_options in RItemEffectOptions::ALL {
                let pregen_entry = RItemEffectsPregenEntry::new(
                    &mut reuse_state_effects,
                    &r_item.base,
                    effects,
                    effect_consts,
                    effect_options,
                );
                let next_id = RItemEffectsPregenId::new(value_ids.len());
                let id = *value_ids.entry(pregen_entry).or_insert(next_id);
                r_item.base.effects_pregen_ids[effect_options.index()] = id;
            }
        }
        let mut id_values: Vec<_> = value_ids.drain().map(|(val, id)| (id, val)).collect();
        id_values.sort_unstable_by_key(|(id, _)| id.index());
        Self {
            data: id_values.into_iter().map(|(_, val)| val).collect(),
        }
    }
}
