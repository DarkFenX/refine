//! Data customizations which are applied on adapted data generation.

use crate::{ad, ntt::NTT_EFFECTS};

mod subsystem_slots;

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    // Effect customization/fixes
    for ntt_effect in NTT_EFFECTS.iter() {
        if let Some(customizer_fn) = ntt_effect.custom_fn_adg {
            customizer_fn(a_data);
        }
    }
    // Attribute value fixes
    subsystem_slots::fix_subsysem_slot_count(a_data);
}
