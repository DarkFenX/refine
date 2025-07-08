//! Data customizations which are applied on adapted data generation.

use crate::{ad, nd::N_EFFECTS};

mod subsystem_slots;

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    // Effect customization/fixes
    for n_effect in N_EFFECTS.iter() {
        if let Some(customizer_fn) = n_effect.adg_custom_fn {
            customizer_fn(a_data);
        }
    }
    // Attribute value fixes
    subsystem_slots::fix_subsysem_slot_count(a_data);
}
