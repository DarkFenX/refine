use std::collections::HashMap;

use ordered_float::OrderedFloat as OF;

use super::shared::is_flag_set;
use crate::{
    ac, ad,
    sol::{
        ItemId, ItemKey,
        svc::{EffectSpec, calc::Calc, eprojs::EProjs, get_resist_mult_val_by_projectee_aspec, vast::VastFitData},
        uad::Uad,
    },
    util::{RMapRSet, RSet},
};

pub struct ValProjImmunityFail {
    /// Map between projecting item IDs and targets they can't be projected to.
    pub items: HashMap<ItemId, Vec<ItemId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_assist_immunity_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            eprojs,
            calc,
            &self.blockable_assistance,
            &ac::attrs::DISALLOW_ASSISTANCE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_offense_immunity_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            eprojs,
            calc,
            &self.blockable_offense,
            &ac::attrs::DISALLOW_OFFENSIVE_MODIFIERS,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_resist_immunity_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> bool {
        for (projectee_aspec, mut projector_especs) in self.resist_immunity.iter() {
            if get_resist_mult_val_by_projectee_aspec(uad, eprojs, calc, projectee_aspec) == Some(OF(0.0)) {
                match kfs.is_empty() {
                    true => return false,
                    false => {
                        if !projector_especs.all(|v| kfs.contains(&v.item_key)) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_assist_immunity_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(
            kfs,
            uad,
            eprojs,
            calc,
            &self.blockable_assistance,
            &ac::attrs::DISALLOW_ASSISTANCE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_offense_immunity_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(
            kfs,
            uad,
            eprojs,
            calc,
            &self.blockable_offense,
            &ac::attrs::DISALLOW_OFFENSIVE_MODIFIERS,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_resist_immunity_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        let mut items = HashMap::new();
        for (projectee_aspec, projector_especs) in self.resist_immunity.iter() {
            if get_resist_mult_val_by_projectee_aspec(uad, eprojs, calc, projectee_aspec) == Some(OF(0.0))
                && !projector_especs.is_empty()
            {
                let projectee_item_id = uad.items.id_by_key(projectee_aspec.item_key);
                for projector_espec in projector_especs {
                    if kfs.contains(&projector_espec.item_key) {
                        continue;
                    }
                    let projector_item_id = uad.items.id_by_key(projector_espec.item_key);
                    let projectee_item_ids = items.entry(projector_item_id).or_insert_with(Vec::new);
                    if !projectee_item_ids.contains(&projectee_item_id) {
                        projectee_item_ids.push(projectee_item_id)
                    }
                }
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValProjImmunityFail { items }),
        }
    }
}

fn validate_fast(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    eprojs: &EProjs,
    calc: &mut Calc,
    blockable: &RMapRSet<ItemKey, EffectSpec>,
    a_attr_id: &ad::AAttrId,
) -> bool {
    for (projectee_item_key, mut projector_especs) in blockable.iter() {
        if is_flag_set(uad, eprojs, calc, *projectee_item_key, a_attr_id) {
            match kfs.is_empty() {
                true => return false,
                false => {
                    if !projector_especs.all(|v| kfs.contains(&v.item_key)) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn validate_verbose(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    eprojs: &EProjs,
    calc: &mut Calc,
    blockable: &RMapRSet<ItemKey, EffectSpec>,
    a_attr_id: &ad::AAttrId,
) -> Option<ValProjImmunityFail> {
    let mut items = HashMap::new();
    for (projectee_item_key, projector_especs) in blockable.iter() {
        if is_flag_set(uad, eprojs, calc, *projectee_item_key, a_attr_id) && !projector_especs.is_empty() {
            let projectee_item_id = uad.items.id_by_key(*projectee_item_key);
            for projector_espec in projector_especs {
                if kfs.contains(&projector_espec.item_key) {
                    continue;
                }
                let projector_item_id = uad.items.id_by_key(projector_espec.item_key);
                let projectee_item_ids = items.entry(projector_item_id).or_insert_with(Vec::new);
                if !projectee_item_ids.contains(&projectee_item_id) {
                    projectee_item_ids.push(projectee_item_id)
                }
            }
        }
    }
    match items.is_empty() {
        true => None,
        false => Some(ValProjImmunityFail { items }),
    }
}
