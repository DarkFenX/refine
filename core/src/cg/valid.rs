use std::collections::HashSet;

use crate::cg::data::Pk;

use super::Data;

pub(super) fn validate(data: &mut Data, errs: &mut Vec<String>) {
    validate_default_effects(data, errs);
}

// Ensure that no item has more than one default effect
fn validate_default_effects(data: &mut Data, errs: &mut Vec<String>) {
    let mut unsets = 0;
    let mut seen_des = HashSet::new();
    for item_effect in data.item_effects.iter_mut() {
        if item_effect.is_default {
            if !seen_des.insert(item_effect.get_pk()) {
                unsets += 1;
                item_effect.is_default = false
            }
        }
    }
    if unsets > 0 {
        let msg = format!("set {} excessive default effects as non-default", unsets);
        log::warn!("{}", &msg);
        errs.push(msg);
    }
}
