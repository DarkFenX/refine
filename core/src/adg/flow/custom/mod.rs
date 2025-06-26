//! Data customizations which are applied on adapted data generation.

use crate::{ad, ntt, ntt::NTT_EFFECTS};

mod drone_dmg_self_srq;
mod missile_dmg_self_srq;
mod missile_rof_self_srq;
mod online_eff_cat;
mod prop_mods;
mod reactive_armor_hardener;
mod rsb;
mod structure_point;
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
    online_eff_cat::fix_online_effect_cat(a_data);
    prop_mods::add_ab_modifiers(a_data);
    prop_mods::add_mwd_modifiers(a_data);
    reactive_armor_hardener::add_rah_modifiers(a_data);
    rsb::add_rsb_modifiers(a_data);
    structure_point::add_structure_point_modifiers(a_data);
    subsystem_mods::add_subsystem_modifiers(a_data);
    wdfg::add_wdfg_modifiers(a_data);
    web::add_drone_web_modifiers(a_data);
    web::add_ship_web_modifiers(a_data);
    web::add_structure_web_modifiers(a_data);
    // Self skill requirement modifiers
    missile_rof_self_srq::mk_self_skillreq_modifiers_launcher_rof(a_data);
    missile_dmg_self_srq::mk_self_skillreq_modifier_missile_dmg(a_data);
    drone_dmg_self_srq::mk_self_skillreq_drone_dmg(a_data);
    // Attribute value fixes
    subsystem_slots::fix_subsysem_slot_count(a_data);
}
