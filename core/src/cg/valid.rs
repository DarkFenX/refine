use std::collections::HashSet;

use crate::cg::data::Pk;

use super::Data;

/// Ensure that assumptions the crate makes about the data are true.
///
/// Cachable type generation and the crate operation relies on several assumptions, which are
/// possible to break with the data handling format the crate exposes.
pub(super) fn validate(data: &mut Data, errs: &mut Vec<String>) {
    default_effects(data, errs);
}

/// Ensure that no item has more than one default effect.
fn default_effects(data: &mut Data, errs: &mut Vec<String>) {
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
