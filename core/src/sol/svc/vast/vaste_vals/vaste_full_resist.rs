use std::collections::HashMap;

use ordered_float::OrderedFloat as OF;

use crate::{
    sol::{
        ItemId, ItemKey,
        svc::{calc::Calc, get_resist_mult_val, vast::VastFitData},
        uad::Uad,
    },
    util::RSet,
};

pub struct ValFullResistFail {
    /// Map between projecting item IDs and targets they can't be projected to.
    pub items: HashMap<ItemId, Vec<ItemId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_full_resist_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        for (projectee_aspec, mut projector_especs) in self.full_resist.iter() {
            if get_resist_mult_val(uad, calc, projectee_aspec) == Some(OF(0.0)) {
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
    pub(in crate::sol::svc::vast) fn validate_full_resist_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValFullResistFail> {
        let mut items = HashMap::new();
        for (projectee_aspec, projector_especs) in self.full_resist.iter() {
            if get_resist_mult_val(uad, calc, projectee_aspec) == Some(OF(0.0)) {
                if !projector_especs.is_empty() {
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
        }
        match items.is_empty() {
            true => None,
            false => Some(ValFullResistFail { items }),
        }
    }
}
