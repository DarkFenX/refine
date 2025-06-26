//! Data customizations which are applied on adapted data generation.

use crate::{ad, ntt::NTT_EFFECTS};

mod subsystem_mods;
mod subsystem_slots;
mod wdfg;
mod web;

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    for ntt_effect in NTT_EFFECTS.iter() {
        if let Some(customizer) = ntt_effect.custom_fn_adg {
            customizer(a_data);
        }
    }
    subsystem_mods::add_subsystem_modifiers(a_data);
    wdfg::add_wdfg_modifiers(a_data);
    web::add_drone_web_modifiers(a_data);
    web::add_ship_web_modifiers(a_data);
    web::add_structure_web_modifiers(a_data);
    // Attribute value fixes
    subsystem_slots::fix_subsysem_slot_count(a_data);
}
