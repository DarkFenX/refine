use std::collections::HashMap;

use super::shared::is_flag_set;
use crate::{
    ac, ad,
    sol::{
        ItemId, ItemKey,
        svc::{EffectSpec, calc::Calc, vast::VastFitData},
        uad::Uad,
    },
    util::{RMapRSet, RSet},
};

pub struct ValEffectImmunityFail {
    /// Map between projecting item IDs and targets they can't be projected to.
    pub items: HashMap<ItemId, Vec<ItemId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_assist_immunity_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &self.blockable_assistance,
            &ac::attrs::DISALLOW_ASSISTANCE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_offense_immunity_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &self.blockable_offense,
            &ac::attrs::DISALLOW_OFFENSIVE_MODIFIERS,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_assist_immunity_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValEffectImmunityFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &self.blockable_assistance,
            &ac::attrs::DISALLOW_ASSISTANCE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_offense_immunity_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValEffectImmunityFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &self.blockable_offense,
            &ac::attrs::DISALLOW_OFFENSIVE_MODIFIERS,
        )
    }
}

fn validate_fast(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    blockable: &RMapRSet<EffectSpec, ItemKey>,
    a_attr_id: &ad::AAttrId,
) -> bool {
    for (projector_spec, projectee_item_keys) in blockable.iter() {
        for &projectee_item_key in projectee_item_keys {
            if is_flag_set(uad, calc, projectee_item_key, a_attr_id) && !kfs.contains(&projector_spec.item_key) {
                return false;
            }
        }
    }
    true
}

fn validate_verbose(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    blockable: &RMapRSet<EffectSpec, ItemKey>,
    a_attr_id: &ad::AAttrId,
) -> Option<ValEffectImmunityFail> {
    let mut items = HashMap::new();
    for (projector_spec, projectee_item_keys) in blockable.iter() {
        for &projectee_item_key in projectee_item_keys {
            if is_flag_set(uad, calc, projectee_item_key, a_attr_id) && !kfs.contains(&projector_spec.item_key) {
                let projectee_item_id = uad.items.id_by_key(projectee_item_key);
                let projectee_item_ids = items
                    .entry(uad.items.id_by_key(projector_spec.item_key))
                    .or_insert_with(Vec::new);
                if !projectee_item_ids.contains(&projectee_item_id) {
                    projectee_item_ids.push(projectee_item_id)
                }
            }
        }
    }
    match items.is_empty() {
        true => None,
        false => Some(ValEffectImmunityFail { items }),
    }
}
