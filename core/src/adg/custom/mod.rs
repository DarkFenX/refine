//! Data customizations which are applied on adapted data generation.

use crate::ad;

mod aar_paste_boost;
mod char_missile_dmg;
mod drone_dmg_self_srq;
mod missile_dmg_self_srq;
mod missile_rof_self_srq;
mod online_eff_cat;
mod prop_mods;
mod subsystem_mods;
mod wubble;

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    online_eff_cat::fix_online_effect_cat(a_data);
    char_missile_dmg::add_char_missile_dmg_mods(a_data);
    subsystem_mods::add_subsystem_modifiers(a_data);
    aar_paste_boost::add_aar_paste_boost_effect(a_data);
    prop_mods::add_ab_modifiers(a_data);
    prop_mods::add_mwd_modifiers(a_data);
    // Self skill requirement modifiers
    missile_rof_self_srq::mk_self_skillreq_modifiers_launcher_rof(a_data);
    missile_dmg_self_srq::mk_self_skillreq_modifier_missile_dmg(a_data);
    drone_dmg_self_srq::mk_self_skillreq_drone_dmg(a_data);
}
